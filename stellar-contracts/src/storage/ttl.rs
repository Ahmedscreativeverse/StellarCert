use soroban_sdk::{Env, storage::StorageKey};

/// Default TTL duration (example: 30 days in ledger blocks)
pub const DEFAULT_TTL: u32 = 30 * 24 * 60 * 60; // adjust based on your block time

/// Extend TTL for a given persistent storage key
pub fn extend_ttl<T: StorageKey>(env: &Env, key: &T, ttl: Option<u32>) {
    let ttl_duration = ttl.unwrap_or(DEFAULT_TTL);
    env.storage().persistent().extend_ttl(key, ttl_duration);
}

/// Extend TTL for a given instance storage key
pub fn extend_instance_ttl<T: StorageKey>(env: &Env, key: &T, ttl: Option<u32>) {
    let ttl_duration = ttl.unwrap_or(DEFAULT_TTL);
    env.storage().instance().extend_ttl(key, ttl_duration);
}

/// Wrapper around instance storage that refreshes TTL on reads and writes.
pub struct TtlInstance<'a> {
    env: &'a Env,
}

impl<'a> TtlInstance<'a> {
    /// Store a value in instance storage and extend its TTL.
    pub fn set<T: StorageKey, V: Clone>(&self, key: &T, value: &V) {
        self.env.storage().instance().set(key, value);
        extend_instance_ttl(self.env, key, None);
    }

    /// Retrieve a value from instance storage and extend its TTL if present.
    pub fn get<T: StorageKey, V: Clone>(&self, key: &T) -> Option<V> {
        let value = self.env.storage().instance().get(key);
        if value.is_some() {
            extend_instance_ttl(self.env, key, None);
        }
        value
    }

    /// Check if a value exists in instance storage and extend its TTL if so.
    pub fn has<T: StorageKey>(&self, key: &T) -> bool {
        let exists = self.env.storage().instance().has(key);
        if exists {
            extend_instance_ttl(self.env, key, None);
        }
        exists
    }

    /// Remove a value from instance storage.
    pub fn remove<T: StorageKey>(&self, key: &T) {
        self.env.storage().instance().remove(key);
    }
}

/// Trait to extend Env with a TTL-aware instance storage helper.
pub trait TtlInstanceExt {
    fn ttl_instance(&self) -> TtlInstance;
}

impl TtlInstanceExt for Env {
    fn ttl_instance(&self) -> TtlInstance {
        TtlInstance { env: self }
    }
}

/// Wrapper around persistent storage that refreshes TTL on reads and writes.
pub struct TtlPersistent<'a> {
    env: &'a Env,
}

impl<'a> TtlPersistent<'a> {
    /// Store a value in persistent storage and extend its TTL.
    pub fn set<T: StorageKey, V: Clone>(&self, key: &T, value: &V) {
        self.env.storage().persistent().set(key, value);
        extend_ttl(self.env, key, None);
    }

    /// Retrieve a value from persistent storage and extend its TTL if present.
    pub fn get<T: StorageKey, V: Clone>(&self, key: &T) -> Option<V> {
        let value = self.env.storage().persistent().get(key);
        if value.is_some() {
            extend_ttl(self.env, key, None);
        }
        value
    }

    /// Check if a value exists in persistent storage and extend its TTL if so.
    pub fn has<T: StorageKey>(&self, key: &T) -> bool {
        let exists = self.env.storage().persistent().has(key);
        if exists {
            extend_ttl(self.env, key, None);
        }
        exists
    }

    /// Remove a value from persistent storage.
    pub fn remove<T: StorageKey>(&self, key: &T) {
        self.env.storage().persistent().remove(key);
    }
}

/// Trait to extend Env with a TTL-aware persistent storage helper.
pub trait TtlPersistentExt {
    fn ttl_persistent(&self) -> TtlPersistent;
}

impl TtlPersistentExt for Env {
    fn ttl_persistent(&self) -> TtlPersistent {
        TtlPersistent { env: self }
    }
}
