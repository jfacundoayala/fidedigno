//! Escrow de ejemplo.
//!
//! Retiene un pago y lo libera solo si el contrato de verificación confirma
//! que la evidencia presentada por el beneficiario está verificada.
//!
//! Es una demostración de integración, no un escrow para uso real: tiene un
//! único hito y no contempla reembolsos.

#![no_std]

use soroban_sdk::{
    contract, contractclient, contracterror, contractevent, contractimpl, contracttype, token,
    Address, BytesN, Env,
};

mod test;

/// Consulta del contrato de verificación que usa este escrow.
#[contractclient(name = "VerificationClient")]
pub trait Verification {
    fn require_verified_for(
        env: Env,
        hash: BytesN<32>,
        issuer: Address,
        min_level: u32,
        min_amount: i128,
    ) -> bool;
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Config,
    State,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub verification: Address,
    pub token: Address,
    pub payer: Address,
    pub payee: Address,
    pub amount: i128,
    pub min_level: u32,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Created,
    Funded,
    Released,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
    NotCreated = 2,
    NotFunded = 3,
    EvidenceNotVerified = 4,
}

#[contractevent]
pub struct Released {
    #[topic]
    pub hash: BytesN<32>,
    pub amount: i128,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn __constructor(env: Env, config: Config) -> Result<(), Error> {
        if config.amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::State, &State::Created);
        Ok(())
    }

    /// El pagador deposita el monto del hito.
    pub fn fund(env: Env) -> Result<(), Error> {
        let config = Self::config(&env);
        if Self::state(env.clone()) != State::Created {
            return Err(Error::NotCreated);
        }
        config.payer.require_auth();

        token::Client::new(&env, &config.token).transfer(
            &config.payer,
            &env.current_contract_address(),
            &config.amount,
        );
        env.storage().instance().set(&DataKey::State, &State::Funded);
        Ok(())
    }

    /// Libera el pago si la evidencia `hash` está verificada para el
    /// beneficiario, con el nivel y el monto de este escrow. Lo puede llamar
    /// cualquiera.
    pub fn release(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        let config = Self::config(&env);
        if Self::state(env.clone()) != State::Funded {
            return Err(Error::NotFunded);
        }

        let verified = VerificationClient::new(&env, &config.verification).require_verified_for(
            &hash,
            &config.payee,
            &config.min_level,
            &config.amount,
        );
        if !verified {
            return Err(Error::EvidenceNotVerified);
        }

        env.storage()
            .instance()
            .set(&DataKey::State, &State::Released);
        token::Client::new(&env, &config.token).transfer(
            &env.current_contract_address(),
            &config.payee,
            &config.amount,
        );

        Released {
            hash,
            amount: config.amount,
        }
        .publish(&env);
        Ok(())
    }

    pub fn state(env: Env) -> State {
        env.storage()
            .instance()
            .get(&DataKey::State)
            .unwrap_or(State::Created)
    }
}

impl EscrowContract {
    fn config(env: &Env) -> Config {
        env.storage().instance().get(&DataKey::Config).unwrap()
    }
}
