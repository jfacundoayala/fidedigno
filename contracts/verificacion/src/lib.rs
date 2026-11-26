//! Contrato de verificación de evidencia.
//!
//! Registra el resultado de verificar una evidencia fuera de la red, retiene
//! una fianza proporcional al riesgo durante un plazo de disputa y expone una
//! consulta para que otros contratos decidan si pueden usar esa evidencia.
//!
//! En la red se guarda solo la huella de la evidencia, nunca el archivo.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, token, Address, BytesN,
    Env,
};

mod test;

/// Escala del índice de confianza (ICE): 10.000 puntos básicos = 1,0.
pub const BPS: u32 = 10_000;

const DAY_IN_LEDGERS: u32 = 17_280;
const TTL_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS;
const TTL_EXTEND_TO: u32 = 60 * DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    MaxAnchorAge,
    Issuer(Address),
    Verifier(Address),
    Arbiter(Address),
    Policy(u32),
    Record(BytesN<32>),
}

/// Política de un nivel de riesgo.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Policy {
    /// ICE mínimo aceptado, en puntos básicos.
    pub min_ice_bps: u32,
    /// Duración del plazo de disputa, en segundos.
    pub window_secs: u64,
    /// Multiplicador de la fianza.
    pub k: u32,
    /// Fianza mínima, en unidades del token.
    pub min_bond: i128,
}

/// Resultado calculado y firmado por el verificador.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationResult {
    /// Índice de confianza de la evidencia, en puntos básicos.
    pub ice_bps: u32,
    /// Huella de las señales usadas en el cálculo.
    pub signals_hash: BytesN<32>,
    /// Versión de la tabla de reglas.
    pub rules_version: u32,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Pending,
    Disputed,
    Verified,
    Rejected,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    pub issuer: Address,
    pub verifier: Address,
    pub amount: i128,
    pub level: u32,
    pub result: VerificationResult,
    pub anchor_ledger: u32,
    pub bond: i128,
    pub deadline: u64,
    pub status: Status,
    pub challenger: Option<Address>,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    IssuerNotRegistered = 1,
    VerifierNotRegistered = 2,
    ArbiterNotRegistered = 3,
    VerifierIsIssuer = 4,
    PolicyNotSet = 5,
    InvalidPolicy = 6,
    InvalidAmount = 7,
    InvalidIce = 8,
    IceBelowMinimum = 9,
    AnchorInFuture = 10,
    AnchorTooOld = 11,
    AlreadySubmitted = 12,
    RecordNotFound = 13,
    NotPending = 14,
    NotDisputed = 15,
    WindowClosed = 16,
    WindowOpen = 17,
    ArbiterIsParty = 18,
    Overflow = 19,
}

#[contractevent]
pub struct Submitted {
    #[topic]
    pub hash: BytesN<32>,
    pub ice_bps: u32,
    pub bond: i128,
    pub deadline: u64,
}

#[contractevent]
pub struct Disputed {
    #[topic]
    pub hash: BytesN<32>,
    pub challenger: Address,
}

#[contractevent]
pub struct Resolved {
    #[topic]
    pub hash: BytesN<32>,
    pub valid: bool,
}

#[contractevent]
pub struct Finalized {
    #[topic]
    pub hash: BytesN<32>,
}

#[contract]
pub struct VerificationContract;

#[contractimpl]
impl VerificationContract {
    /// `token` es el activo de las fianzas. `max_anchor_age` es la cantidad
    /// máxima de ledgers entre el ancla de la captura y el envío.
    pub fn __constructor(env: Env, admin: Address, token: Address, max_anchor_age: u32) {
        let storage = env.storage().instance();
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::Token, &token);
        storage.set(&DataKey::MaxAnchorAge, &max_anchor_age);
    }

    pub fn register_issuer(env: Env, issuer: Address) {
        Self::require_admin(&env);
        Self::set_role(&env, &DataKey::Issuer(issuer));
    }

    pub fn register_verifier(env: Env, verifier: Address) {
        Self::require_admin(&env);
        Self::set_role(&env, &DataKey::Verifier(verifier));
    }

    pub fn register_arbiter(env: Env, arbiter: Address) {
        Self::require_admin(&env);
        Self::set_role(&env, &DataKey::Arbiter(arbiter));
    }

    pub fn set_policy(env: Env, level: u32, policy: Policy) -> Result<(), Error> {
        Self::require_admin(&env);
        if policy.min_ice_bps > BPS || policy.k == 0 || policy.min_bond <= 0 {
            return Err(Error::InvalidPolicy);
        }
        env.storage().instance().set(&DataKey::Policy(level), &policy);
        Self::extend_instance(&env);
        Ok(())
    }

    /// Registra una evidencia verificada y retiene la fianza del emisor.
    ///
    /// Requiere la firma del emisor y la del verificador, que tienen que ser
    /// cuentas distintas. La fianza es `max(min_bond, k × (1 − ICE) × amount)`.
    pub fn submit(
        env: Env,
        hash: BytesN<32>,
        issuer: Address,
        verifier: Address,
        amount: i128,
        level: u32,
        result: VerificationResult,
        anchor_ledger: u32,
    ) -> Result<i128, Error> {
        if issuer == verifier {
            return Err(Error::VerifierIsIssuer);
        }
        issuer.require_auth();
        verifier.require_auth();

        if !Self::has_role(&env, &DataKey::Issuer(issuer.clone())) {
            return Err(Error::IssuerNotRegistered);
        }
        if !Self::has_role(&env, &DataKey::Verifier(verifier.clone())) {
            return Err(Error::VerifierNotRegistered);
        }
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if result.ice_bps > BPS {
            return Err(Error::InvalidIce);
        }

        let policy: Policy = env
            .storage()
            .instance()
            .get(&DataKey::Policy(level))
            .ok_or(Error::PolicyNotSet)?;
        if result.ice_bps < policy.min_ice_bps {
            return Err(Error::IceBelowMinimum);
        }

        let current = env.ledger().sequence();
        if anchor_ledger > current {
            return Err(Error::AnchorInFuture);
        }
        let max_age: u32 = env
            .storage()
            .instance()
            .get(&DataKey::MaxAnchorAge)
            .unwrap_or(0);
        if current - anchor_ledger > max_age {
            return Err(Error::AnchorTooOld);
        }

        let key = DataKey::Record(hash.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::AlreadySubmitted);
        }

        let bond = Self::bond_for(&policy, result.ice_bps, amount)?;
        Self::token(&env).transfer(&issuer, &env.current_contract_address(), &bond);

        let deadline = env.ledger().timestamp() + policy.window_secs;
        let record = Record {
            issuer,
            verifier,
            amount,
            level,
            result: result.clone(),
            anchor_ledger,
            bond,
            deadline,
            status: Status::Pending,
            challenger: None,
        };
        Self::save_record(&env, &key, &record);
        Self::extend_instance(&env);

        Submitted {
            hash,
            ice_bps: result.ice_bps,
            bond,
            deadline,
        }
        .publish(&env);
        Ok(bond)
    }

    /// Abre una disputa dentro del plazo. Quien disputa deposita una fianza
    /// igual a la del emisor.
    pub fn dispute(env: Env, hash: BytesN<32>, challenger: Address) -> Result<(), Error> {
        challenger.require_auth();

        let key = DataKey::Record(hash.clone());
        let mut record = Self::load_record(&env, &key)?;
        if record.status != Status::Pending {
            return Err(Error::NotPending);
        }
        if env.ledger().timestamp() >= record.deadline {
            return Err(Error::WindowClosed);
        }

        Self::token(&env).transfer(&challenger, &env.current_contract_address(), &record.bond);

        record.status = Status::Disputed;
        record.challenger = Some(challenger.clone());
        Self::save_record(&env, &key, &record);

        Disputed { hash, challenger }.publish(&env);
        Ok(())
    }

    /// Resuelve una disputa. Quien gana recibe las dos fianzas.
    pub fn resolve(env: Env, hash: BytesN<32>, arbiter: Address, valid: bool) -> Result<(), Error> {
        arbiter.require_auth();
        if !Self::has_role(&env, &DataKey::Arbiter(arbiter.clone())) {
            return Err(Error::ArbiterNotRegistered);
        }

        let key = DataKey::Record(hash.clone());
        let mut record = Self::load_record(&env, &key)?;
        if record.status != Status::Disputed {
            return Err(Error::NotDisputed);
        }
        let challenger = record.challenger.clone().ok_or(Error::NotDisputed)?;
        if arbiter == record.issuer || arbiter == record.verifier || arbiter == challenger {
            return Err(Error::ArbiterIsParty);
        }

        let total = record.bond.checked_mul(2).ok_or(Error::Overflow)?;
        let winner = if valid { &record.issuer } else { &challenger };
        Self::token(&env).transfer(&env.current_contract_address(), winner, &total);

        record.status = if valid {
            Status::Verified
        } else {
            Status::Rejected
        };
        Self::save_record(&env, &key, &record);

        Resolved { hash, valid }.publish(&env);
        Ok(())
    }

    /// Cierra una evidencia sin disputa una vez vencido el plazo y devuelve
    /// la fianza al emisor. Lo puede llamar cualquiera.
    pub fn finalize(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        let key = DataKey::Record(hash.clone());
        let mut record = Self::load_record(&env, &key)?;
        if record.status != Status::Pending {
            return Err(Error::NotPending);
        }
        if env.ledger().timestamp() < record.deadline {
            return Err(Error::WindowOpen);
        }

        Self::token(&env).transfer(&env.current_contract_address(), &record.issuer, &record.bond);

        record.status = Status::Verified;
        Self::save_record(&env, &key, &record);

        Finalized { hash }.publish(&env);
        Ok(())
    }

    /// Devuelve `true` solo si la evidencia está verificada y su nivel es al
    /// menos `min_level`. Si no hay registro, devuelve `false`.
    pub fn require_verified(env: Env, hash: BytesN<32>, min_level: u32) -> bool {
        match Self::get(env, hash) {
            Some(record) => record.status == Status::Verified && record.level >= min_level,
            None => false,
        }
    }

    /// Como `require_verified`, pero además exige que la evidencia sea de
    /// `issuer` y que se haya verificado por un monto de al menos
    /// `min_amount`. Evita que una evidencia con fianza chica habilite un
    /// pago grande.
    pub fn require_verified_for(
        env: Env,
        hash: BytesN<32>,
        issuer: Address,
        min_level: u32,
        min_amount: i128,
    ) -> bool {
        match Self::get(env, hash) {
            Some(record) => {
                record.status == Status::Verified
                    && record.level >= min_level
                    && record.issuer == issuer
                    && record.amount >= min_amount
            }
            None => false,
        }
    }

    pub fn get(env: Env, hash: BytesN<32>) -> Option<Record> {
        env.storage().persistent().get(&DataKey::Record(hash))
    }

    pub fn get_policy(env: Env, level: u32) -> Option<Policy> {
        env.storage().instance().get(&DataKey::Policy(level))
    }
}

impl VerificationContract {
    fn require_admin(env: &Env) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
    }

    fn token(env: &Env) -> token::Client<'_> {
        let address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        token::Client::new(env, &address)
    }

    fn set_role(env: &Env, key: &DataKey) {
        env.storage().persistent().set(key, &true);
        env.storage()
            .persistent()
            .extend_ttl(key, TTL_THRESHOLD, TTL_EXTEND_TO);
        Self::extend_instance(env);
    }

    fn has_role(env: &Env, key: &DataKey) -> bool {
        env.storage().persistent().has(key)
    }

    fn load_record(env: &Env, key: &DataKey) -> Result<Record, Error> {
        env.storage()
            .persistent()
            .get(key)
            .ok_or(Error::RecordNotFound)
    }

    fn save_record(env: &Env, key: &DataKey, record: &Record) {
        env.storage().persistent().set(key, record);
        env.storage()
            .persistent()
            .extend_ttl(key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    fn extend_instance(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
    }

    /// `max(min_bond, k × (BPS − ice) × amount / BPS)`.
    fn bond_for(policy: &Policy, ice_bps: u32, amount: i128) -> Result<i128, Error> {
        let risk = (BPS - ice_bps) as i128;
        let proportional = (policy.k as i128)
            .checked_mul(risk)
            .and_then(|v| v.checked_mul(amount))
            .ok_or(Error::Overflow)?
            / BPS as i128;
        Ok(proportional.max(policy.min_bond))
    }
}
