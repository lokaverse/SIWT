//! # SIWT (Sign In With Telegram) Backend Canister
//!
//! This canister provides authentication and identity management services for Telegram users
//! on the Internet Computer. It enables secure delegation-based authentication without
//! requiring users to manage private keys directly.
//!
//! ## Security Considerations
//!
//! **Important**: This implementation lacks a message signing step for Telegram ID ownership
//! verification, which makes it less secure than SIWB (Sign In With Bitcoin). Users should
//! be aware that Telegram ID ownership cannot be cryptographically verified.
//!
//! ## Features
//!
//! - User authentication via Telegram integration
//! - Delegation-based identity management
//! - Account derivation for various blockchain networks
//! - Global state management for application data
//! - Configurable expiration settings
//!
//! ## API Overview
//!
//! The canister exposes both query and update methods for:
//! - Authentication and delegation
//! - Account management
//! - Global state operations
//! - Configuration management

use ic_cdk::{init, query, update};
use types::*;

mod accounts;
mod canisters;
mod delegation;
mod globals;
mod messages;
mod payloads;
mod responses;
mod services;
mod setting;
mod signatures;
mod state;
mod timestamp;
mod types;

pub mod hash;
pub mod states;
pub mod utils;

pub use crate::accounts::Accounts;
pub use crate::delegation::Delegation;
pub use crate::globals::Globals;
pub use crate::messages::{Message, Messages};
pub use crate::setting::Setting;
pub use crate::signatures::Signatures;
pub use crate::state::{State, LABEL_ASSETS, LABEL_SIG};
pub use crate::timestamp::Timestamp;

/// Checks if the current caller is authorized to access protected endpoints.
///
/// This function verifies that the caller's principal is included in the
/// authorized principals list stored in the canister's settings.
///
/// # Returns
///
/// * `Ok(())` - If the caller is authorized
/// * `Err(String)` - If the caller is not authorized, with error message "Unauthorized"
///
/// # Security Note
///
/// This is a critical security function that gates access to sensitive operations.
/// Only principals explicitly added to the authorized list can access protected endpoints.
pub fn authorized() -> Result<(), String> {
    if states::setting::get().authorized(&caller_principal()) {
        return Ok(());
    }

    Err("Unauthorized".to_owned())
}

/// Initializes the canister with the provided settings.
///
/// This function is called once when the canister is first deployed.
/// It merges the provided settings with any existing configuration.
///
/// # Arguments
///
/// * `setting` - The initial configuration settings for the canister
///
/// # Note
///
/// This function can only be called during canister initialization.
#[init]
pub async fn init(setting: Setting) {
    setting.merge();
}

/// Returns a map of available features and their enabled status.
///
/// This query method allows clients to discover which optional features
/// are compiled into this canister instance.
///
/// # Returns
///
/// A map where keys are feature names and values indicate if the feature is enabled:
/// * `"ckbtc"` - Whether ckBTC integration is available
///
/// # Example Response
///
/// ```json
/// {
///   "ckbtc": true
/// }
/// ```
#[query]
pub async fn features() -> Map<&'static str, bool> {
    let mut features = Map::new();

    features.insert("ckbtc", cfg!(feature = "ckbtc"));
    features
}

/// Retrieves the current canister settings.
///
/// This query method returns the complete configuration settings for the canister,
/// including authorized principals, expiration settings, and other configuration data.
///
/// # Authorization
///
/// This endpoint requires authorization. Only principals in the authorized list can access it.
///
/// # Returns
///
/// The current `Setting` configuration object containing all canister settings.
#[query(name = "setting", guard = "authorized")]
pub async fn setting() -> Setting {
    states::setting::get()
}

/// Extends the current canister settings with additional configuration.
///
/// This update method allows authorized callers to modify the canister's settings
/// by extending the current configuration with new values.
///
/// # Arguments
///
/// * `payload` - A `SettingExtendsPayload` containing the settings to merge
///
/// # Authorization
///
/// This endpoint requires authorization. Only principals in the authorized list can modify settings.
///
/// # Behavior
///
/// The provided settings are merged with existing settings, with new values
/// taking precedence over existing ones.
#[update(name = "extends", guard = "authorized")]
pub async fn extends(payload: payloads::SettingExtendsPayload) {
    let mut setting = states::setting::get();
    setting.extends(payload.authorities, payload.canisters);
    setting.store();
}

/// Sets the expiration time in minutes for delegations.
///
/// This update method configures how long delegations remain valid before expiring.
///
/// # Arguments
///
/// * `minute` - The expiration time in minutes for new delegations
///
/// # Authorization
///
/// This endpoint requires authorization. Only principals in the authorized list can modify settings.
///
/// # Security Note
///
/// Shorter expiration times improve security by limiting the window of potential misuse,
/// but may require more frequent re-authentication.
#[update(name = "setExpirationMinute", guard = "authorized")]
pub async fn set_expiration_minute(minute: u64) {
    let mut setting = states::setting::get();
    setting.set_expiration_minute(minute);
    setting.store();
}

#[update(name = "accountDerivedAddress", guard = "authorized")]
async fn account_derived_address(
    payload: payloads::AccountDerivedAddressPayload,
) -> responses::AccountDerivedAddressResponse {
    services::accounts::derived(payload).await.into()
}

#[query(guard = "authorized")]
async fn principal(user: String) -> Option<Principal> {
    states::accounts::borrow(|accounts| accounts.principal(&user))
}

#[query(guard = "authorized")]
async fn user(principal: Principal) -> Option<String> {
    states::accounts::borrow(|accounts| accounts.user(&principal))
}

#[query(guard = "authorized")]
async fn all() -> Map<String, Principal> {
    states::accounts::borrow(|accounts| accounts.all().collect())
}

#[query]
async fn caller() -> (Principal, Option<String>) {
    let principal = ic_cdk::caller();
    let user = states::accounts::borrow(|accounts| accounts.user(&principal));

    (principal, user)
}

#[update(guard = "authorized")]
async fn prepare(payload: payloads::PreparePayload) -> responses::PrepareResponse {
    services::auth::prepare(payload).await.into()
}

#[update(guard = "authorized")]
async fn login(payload: payloads::LoginPayload) -> responses::LoginResponse {
    services::auth::login(payload).await.into()
}

#[query(guard = "authorized")]
async fn delegation(payload: payloads::DelegationPayload) -> responses::SignedDelegationResponse {
    let certificate = data_certificate().expect("delegation must be called using a query call");

    services::auth::delegation(certificate.into(), payload)
        .await
        .into()
}

#[query(name = "globals", guard = "authorized")]
async fn globals() -> Map<String, ByteBuf> {
    services::globals::all().await.collect()
}

#[query(name = "globalsGet", guard = "authorized")]
async fn globals_get(key: String) -> Option<ByteBuf> {
    services::globals::get(key).await
}

#[query(name = "globalsIn", guard = "authorized")]
async fn globals_in(keys: Set<String>) -> Map<String, ByteBuf> {
    services::globals::of(keys.into_iter()).await.collect()
}

#[query(name = "globalsKeys", guard = "authorized")]
async fn globals_keys() -> Set<String> {
    services::globals::keys().await.collect()
}

#[query(name = "globalsHas", guard = "authorized")]
async fn globals_has(key: String) -> bool {
    services::globals::has(key).await
}

#[query(name = "globalsContains", guard = "authorized")]
async fn globals_contains(keys: Set<String>) -> bool {
    services::globals::contains(keys.into_iter()).await
}

#[update(name = "globalsStore", guard = "authorized")]
async fn globals_store(key: String, value: ByteBuf) {
    services::globals::store(key, value).await
}

#[update(name = "globalsStores", guard = "authorized")]
async fn globals_stores(values: Map<String, ByteBuf>) -> Map<String, ByteBuf> {
    services::globals::stores(values.into_iter())
        .await
        .collect()
}

#[update(name = "globalsRemove", guard = "authorized")]
async fn globals_remove(key: String) -> Option<ByteBuf> {
    services::globals::remove(key).await
}

#[update(name = "globalsRemoves", guard = "authorized")]
async fn globals_removes(keys: Set<String>) -> Map<String, ByteBuf> {
    services::globals::removes(keys.into_iter()).await.collect()
}

ic_cdk::export_candid!();
