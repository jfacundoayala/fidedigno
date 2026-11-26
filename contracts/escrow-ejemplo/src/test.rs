#![cfg(test)]

use super::*;
use fidedigno_verificacion::{
    Policy, VerificationContract, VerificationContractClient, VerificationResult,
};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{StellarAssetClient, TokenClient},
    Address, BytesN, Env,
};

const UNIT: i128 = 10_000_000;
const AMOUNT: i128 = 100 * UNIT;
const MEDIUM: u32 = 2;
const DAY: u64 = 86_400;
const START_LEDGER: u32 = 1_000;

struct Setup<'a> {
    env: Env,
    verification: VerificationContractClient<'a>,
    escrow: EscrowContractClient<'a>,
    token: TokenClient<'a>,
    payee: Address,
    verifier: Address,
}

fn setup<'a>() -> Setup<'a> {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|l| {
        l.sequence_number = START_LEDGER;
        l.timestamp = 1_800_000_000;
    });

    let admin = Address::generate(&env);
    let payer = Address::generate(&env);
    let payee = Address::generate(&env);
    let verifier = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let token = TokenClient::new(&env, &sac.address());
    let asset_admin = StellarAssetClient::new(&env, &sac.address());
    asset_admin.mint(&payer, &AMOUNT);
    asset_admin.mint(&payee, &(50 * UNIT));

    let verification_id = env.register(VerificationContract, (admin, sac.address(), 20u32));
    let verification = VerificationContractClient::new(&env, &verification_id);
    verification.register_issuer(&payee);
    verification.register_verifier(&verifier);
    verification.set_policy(
        &MEDIUM,
        &Policy {
            min_ice_bps: 8_000,
            window_secs: DAY,
            k: 2,
            min_bond: UNIT / 10,
        },
    );

    let escrow_id = env.register(
        EscrowContract,
        (Config {
            verification: verification_id,
            token: sac.address(),
            payer,
            payee: payee.clone(),
            amount: AMOUNT,
            min_level: MEDIUM,
        },),
    );
    let escrow = EscrowContractClient::new(&env, &escrow_id);
    escrow.fund();

    Setup {
        env,
        verification,
        escrow,
        token,
        payee,
        verifier,
    }
}

fn submit(s: &Setup, hash: &BytesN<32>, amount: i128) {
    s.verification.submit(
        hash,
        &s.payee,
        &s.verifier,
        &amount,
        &MEDIUM,
        &VerificationResult {
            ice_bps: 9_677,
            signals_hash: BytesN::from_array(&s.env, &[0xAA; 32]),
            rules_version: 1,
        },
        &START_LEDGER,
    );
}

fn close_window(s: &Setup, hash: &BytesN<32>) {
    s.env.ledger().with_mut(|l| l.timestamp += DAY);
    s.verification.finalize(hash);
}

#[test]
fn paga_solo_cuando_la_evidencia_queda_verificada() {
    let s = setup();
    let hash = BytesN::from_array(&s.env, &[1; 32]);
    let before = s.token.balance(&s.payee);

    // Sin evidencia registrada.
    assert_eq!(
        s.escrow.try_release(&hash),
        Err(Ok(Error::EvidenceNotVerified))
    );

    // Evidencia enviada, con el plazo de disputa todavía abierto.
    submit(&s, &hash, AMOUNT);
    assert_eq!(
        s.escrow.try_release(&hash),
        Err(Ok(Error::EvidenceNotVerified))
    );

    close_window(&s, &hash);
    s.escrow.release(&hash);

    assert_eq!(s.escrow.state(), State::Released);
    assert_eq!(s.token.balance(&s.payee), before + AMOUNT);
    assert_eq!(s.escrow.try_release(&hash), Err(Ok(Error::NotFunded)));
}

#[test]
fn no_paga_con_evidencia_verificada_por_un_monto_menor() {
    let s = setup();
    let hash = BytesN::from_array(&s.env, &[2; 32]);

    submit(&s, &hash, UNIT);
    close_window(&s, &hash);

    assert_eq!(
        s.escrow.try_release(&hash),
        Err(Ok(Error::EvidenceNotVerified))
    );
}
