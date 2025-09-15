use ic_cdk::{init, query, update};
use types::*;

mod accounts;
mod canisters;
mod delegation;
mod globals;
mod hash;
mod messages;
mod middlewares;
mod payloads;
mod responses;
mod services;
mod setting;
mod signatures;
mod state;
mod states;
mod timestamp;
mod types;
mod utils;

fn authorized() -> Result<(), String> {
    if states::setting::get().authorized(&caller_principal()) {
        return Ok(());
    }

    Err("Unauthorized".to_owned())
}

#[init]
async fn init(setting: Setting) {
    setting.merge();
}

#[query]
async fn features() -> Map<&'static str, bool> {
    let mut features = Map::new();

    features.insert("ckbtc", cfg!(feature = "ckbtc"));
    features
}

#[query(name = "setting", guard = "authorized")]
async fn setting() -> Setting {
    states::setting::get()
}

#[update(name = "extends", guard = "authorized")]
async fn extends(payload: payloads::SettingExtendsPayload) {
    let mut setting = states::setting::get();

    setting.extends(payload.authorities, payload.canisters);
    setting.store();
}

#[update(name = "setExpirationMinute", guard = "authorized")]
async fn set_expiration_minute(minute: u64) {
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
