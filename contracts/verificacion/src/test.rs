#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{StellarAssetClient, TokenClient},
    Address, BytesN, Env,
};

const UNIT: i128 = 10_000_000; // 1 unidad del token, con 7 decimales
const START_LEDGER: u32 = 1_000;
const START_TIME: u64 = 1_800_000_000;
const DAY: u64 = 86_400;
const MAX_ANCHOR_AGE: u32 = 20;

const LOW: u32 = 1;
const MEDIUM: u32 = 2;
const HIGH: u32 = 3;

struct Setup<'a> {
    env: Env,
    client: VerificationContractClient<'a>,
    token: TokenClient<'a>,
    issuer: Address,
    verifier: Address,
    arbiter: Address,
    challenger: Address,
}

fn setup<'a>() -> Setup<'a> {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|l| {
        l.sequence_number = START_LEDGER;
        l.timestamp = START_TIME;
    });

    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);
    let verifier = Address::generate(&env);
    let arbiter = Address::generate(&env);
    let challenger = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let token = TokenClient::new(&env, &sac.address());
    let asset_admin = StellarAssetClient::new(&env, &sac.address());
    asset_admin.mint(&issuer, &(1_000 * UNIT));
    asset_admin.mint(&challenger, &(1_000 * UNIT));

    let id = env.register(
        VerificationContract,
        (admin.clone(), sac.address(), MAX_ANCHOR_AGE),
    );
    let client = VerificationContractClient::new(&env, &id);

    client.register_issuer(&issuer);
    client.register_verifier(&verifier);
    client.register_arbiter(&arbiter);
    client.set_policy(&LOW, &policy(5_000, 2 * 3_600, 1));
    client.set_policy(&MEDIUM, &policy(8_000, DAY, 2));
    client.set_policy(&HIGH, &policy(9_500, 3 * DAY, 3));

    Setup {
        env,
        client,
        token,
        issuer,
        verifier,
        arbiter,
        challenger,
    }
}

fn policy(min_ice_bps: u32, window_secs: u64, k: u32) -> Policy {
    Policy {
        min_ice_bps,
        window_secs,
        k,
        min_bond: UNIT / 10,
    }
}

fn hash(env: &Env, n: u8) -> BytesN<32> {
    BytesN::from_array(env, &[n; 32])
}

fn result(env: &Env, ice_bps: u32) -> VerificationResult {
    VerificationResult {
        ice_bps,
        signals_hash: hash(env, 0xAA),
        rules_version: 1,
    }
}

fn advance(env: &Env, secs: u64) {
    env.ledger().with_mut(|l| {
        l.timestamp += secs;
        l.sequence_number += (secs / 5) as u32;
    });
}

#[test]
fn evidencia_sin_disputa_queda_verificada_y_devuelve_la_fianza() {
    let s = setup();
    let h = hash(&s.env, 1);
    let before = s.token.balance(&s.issuer);

    // ICE 0,9677, nivel medio (k = 2), hito de 100: fianza = 2 × 0,0323 × 100 = 6,46.
    let bond = s.client.submit(
        &h,
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &START_LEDGER,
    );
    assert_eq!(bond, 64_600_000);
    assert_eq!(s.token.balance(&s.issuer), before - bond);
    assert!(!s.client.require_verified(&h, &MEDIUM));
    assert_eq!(s.client.try_finalize(&h), Err(Ok(Error::WindowOpen)));

    advance(&s.env, DAY);
    s.client.finalize(&h);

    assert!(s.client.require_verified(&h, &MEDIUM));
    assert_eq!(s.token.balance(&s.issuer), before);
    assert_eq!(s.client.get(&h).unwrap().status, Status::Verified);
}

#[test]
fn a_menor_confianza_la_fianza_es_mayor() {
    let s = setup();

    // ICE 0,7826, nivel bajo (k = 1), hito de 100: fianza = 0,2174 × 100 = 21,74.
    let bond = s.client.submit(
        &hash(&s.env, 1),
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &LOW,
        &result(&s.env, 7_826),
        &START_LEDGER,
    );
    assert_eq!(bond, 217_400_000);
}

#[test]
fn se_aplica_la_fianza_minima() {
    let s = setup();

    // ICE 1,0: la fianza proporcional es 0 y rige la mínima.
    let bond = s.client.submit(
        &hash(&s.env, 1),
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &HIGH,
        &result(&s.env, 10_000),
        &START_LEDGER,
    );
    assert_eq!(bond, UNIT / 10);
}

#[test]
fn el_verificador_no_puede_ser_el_emisor() {
    let s = setup();
    s.client.register_verifier(&s.issuer);

    let res = s.client.try_submit(
        &hash(&s.env, 1),
        &s.issuer,
        &s.issuer,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &START_LEDGER,
    );
    assert_eq!(res, Err(Ok(Error::VerifierIsIssuer)));
}

#[test]
fn rechaza_emisor_o_verificador_sin_registrar() {
    let s = setup();
    let stranger = Address::generate(&s.env);

    let res = s.client.try_submit(
        &hash(&s.env, 1),
        &stranger,
        &s.verifier,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &START_LEDGER,
    );
    assert_eq!(res, Err(Ok(Error::IssuerNotRegistered)));

    let res = s.client.try_submit(
        &hash(&s.env, 1),
        &s.issuer,
        &stranger,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &START_LEDGER,
    );
    assert_eq!(res, Err(Ok(Error::VerifierNotRegistered)));
}

#[test]
fn rechaza_un_ice_menor_al_minimo_del_nivel() {
    let s = setup();

    let res = s.client.try_submit(
        &hash(&s.env, 1),
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 7_826),
        &START_LEDGER,
    );
    assert_eq!(res, Err(Ok(Error::IceBelowMinimum)));
}

#[test]
fn rechaza_un_ancla_vieja_o_futura() {
    let s = setup();

    let old = s.client.try_submit(
        &hash(&s.env, 1),
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &(START_LEDGER - MAX_ANCHOR_AGE - 1),
    );
    assert_eq!(old, Err(Ok(Error::AnchorTooOld)));

    let future = s.client.try_submit(
        &hash(&s.env, 1),
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &(START_LEDGER + 1),
    );
    assert_eq!(future, Err(Ok(Error::AnchorInFuture)));
}

#[test]
fn una_huella_no_se_puede_enviar_dos_veces() {
    let s = setup();
    let h = hash(&s.env, 1);
    let submit = || {
        s.client.try_submit(
            &h,
            &s.issuer,
            &s.verifier,
            &(100 * UNIT),
            &MEDIUM,
            &result(&s.env, 9_677),
            &START_LEDGER,
        )
    };

    assert!(submit().is_ok());
    assert_eq!(submit(), Err(Ok(Error::AlreadySubmitted)));
}

#[test]
fn disputa_ganada_por_quien_disputa() {
    let s = setup();
    let h = hash(&s.env, 1);
    let issuer_before = s.token.balance(&s.issuer);
    let challenger_before = s.token.balance(&s.challenger);

    let bond = s.client.submit(
        &h,
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &START_LEDGER,
    );
    s.client.dispute(&h, &s.challenger);
    assert_eq!(s.client.try_finalize(&h), Err(Ok(Error::NotPending)));

    s.client.resolve(&h, &s.arbiter, &false);

    assert_eq!(s.client.get(&h).unwrap().status, Status::Rejected);
    assert!(!s.client.require_verified(&h, &LOW));
    assert_eq!(s.token.balance(&s.issuer), issuer_before - bond);
    assert_eq!(s.token.balance(&s.challenger), challenger_before + bond);
}

#[test]
fn disputa_ganada_por_el_emisor() {
    let s = setup();
    let h = hash(&s.env, 1);
    let issuer_before = s.token.balance(&s.issuer);

    let bond = s.client.submit(
        &h,
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &START_LEDGER,
    );
    s.client.dispute(&h, &s.challenger);
    s.client.resolve(&h, &s.arbiter, &true);

    assert!(s.client.require_verified(&h, &MEDIUM));
    assert_eq!(s.token.balance(&s.issuer), issuer_before + bond);
}

#[test]
fn no_se_puede_disputar_con_el_plazo_vencido() {
    let s = setup();
    let h = hash(&s.env, 1);

    s.client.submit(
        &h,
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &START_LEDGER,
    );
    advance(&s.env, DAY);

    assert_eq!(
        s.client.try_dispute(&h, &s.challenger),
        Err(Ok(Error::WindowClosed))
    );
}

#[test]
fn el_arbitro_no_puede_ser_parte_ni_estar_sin_registrar() {
    let s = setup();
    let h = hash(&s.env, 1);

    s.client.submit(
        &h,
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &MEDIUM,
        &result(&s.env, 9_677),
        &START_LEDGER,
    );
    s.client.dispute(&h, &s.challenger);

    s.client.register_arbiter(&s.challenger);
    assert_eq!(
        s.client.try_resolve(&h, &s.challenger, &false),
        Err(Ok(Error::ArbiterIsParty))
    );

    let stranger = Address::generate(&s.env);
    assert_eq!(
        s.client.try_resolve(&h, &stranger, &true),
        Err(Ok(Error::ArbiterNotRegistered))
    );
}

#[test]
fn la_consulta_respeta_nivel_emisor_y_monto() {
    let s = setup();
    let h = hash(&s.env, 1);

    s.client.submit(
        &h,
        &s.issuer,
        &s.verifier,
        &(100 * UNIT),
        &LOW,
        &result(&s.env, 7_826),
        &START_LEDGER,
    );
    advance(&s.env, 2 * 3_600);
    s.client.finalize(&h);

    assert!(s.client.require_verified(&h, &LOW));
    assert!(!s.client.require_verified(&h, &MEDIUM));
    assert!(!s.client.require_verified(&hash(&s.env, 9), &LOW));

    let amount = 100 * UNIT;
    assert!(s.client.require_verified_for(&h, &s.issuer, &LOW, &amount));
    assert!(!s.client.require_verified_for(&h, &s.issuer, &LOW, &(amount + 1)));
    assert!(!s.client.require_verified_for(&h, &s.challenger, &LOW, &amount));
}
