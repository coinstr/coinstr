// Copyright (c) 2022-2023 Coinstr
// Distributed under the MIT software license

mod abortable;
mod address;
mod amount;
mod balance;
mod client;
mod config;
mod contact;
mod descriptor;
mod error;
mod logger;
mod message;
mod network;
mod nip46;
mod policy;
mod proposal;
mod seed;
mod signer;
mod transaction;

use self::error::Result;

pub fn git_hash_version() -> String {
    env!("GIT_HASH").to_string()
}

pub fn get_keychains_list(base_path: String, network: Network) -> Result<Vec<String>> {
    Ok(coinstr_sdk::Coinstr::list_keychains(
        base_path,
        network.into(),
    )?)
}

mod ffi {
    // Error
    pub use crate::error::FFIError;

    // External
    pub use coinstr_sdk::core::policy::PolicyTemplateType;
    pub use coinstr_sdk::core::signer::SignerType;
    pub use coinstr_sdk::core::types::WordCount;
    pub use coinstr_sdk::nostr::RelayStatus;
    pub use nostr_sdk_ffi::{
        EventId, Keys, Metadata, NostrConnectURI, PublicKey, Relay, RelayConnectionStats,
        RelayInformationDocument, SecretKey, Timestamp,
    };

    // Namespace
    pub use crate::logger::init_logger;
    pub use crate::{get_keychains_list, git_hash_version};

    // Coinstr
    pub use crate::abortable::AbortHandle;
    pub use crate::address::{AddressIndex, GetAddress};
    pub use crate::amount::Amount;
    pub use crate::balance::Balance;
    pub use crate::client::{Coinstr, SyncHandler};
    pub use crate::config::Config;
    pub use crate::contact::GetContact;
    pub use crate::descriptor::Descriptor;
    pub use crate::message::{EventHandled, Message};
    pub use crate::network::Network;
    pub use crate::nip46::{NostrConnectRequest, NostrConnectSession};
    pub use crate::policy::{
        AbsoluteLockTime, GetPolicy, Policy, PolicyTemplate, RecoveryTemplate, RelativeLockTime,
    };
    pub use crate::proposal::{
        ApprovedProposal, CompletedProposal, GetApproval, GetCompletedProposal, GetProposal,
        Proposal,
    };
    pub use crate::seed::Seed as KeychainSeed;
    pub use crate::signer::{GetSharedSigner, GetSigner, SharedSigner, Signer};
    pub use crate::transaction::{
        BlockTime, GetTransaction, OutPoint, Transaction, TransactionDetails, TxIn, TxOut, Utxo,
    };

    // UDL
    uniffi::include_scaffolding!("coinstr_sdk");
}
pub use ffi::*;
