use crate::payloads::AccountDerivedAddressPayload;
use crate::responses::AccountDerivedAddress;
use crate::types::{Accounts, Principal};

#[cfg(not(feature = "ckbtc"))]
pub async fn derived(
    payload: AccountDerivedAddressPayload,
) -> Result<AccountDerivedAddress, String> {
    let pubkey = Accounts::user_canister_public_key(&payload.user)?;
    let principal = Principal::self_authenticating(&pubkey);

    Ok(AccountDerivedAddress { pubkey, principal })
}

#[cfg(feature = "ckbtc")]
pub async fn derived(
    payload: AccountDerivedAddressPayload,
) -> Result<AccountDerivedAddress, String> {
    use crate::responses::AccountDerivedBtcAddress;
    use crate::services::ckbtc;
    use futures::future::join_all;

    let pubkey = Accounts::user_canister_public_key(&payload.user)?;
    let principal = Principal::self_authenticating(&pubkey);

    let address = ckbtc::owned(principal);
    let accounts = join_all(
        payload
            .ckbtc
            .owners
            .iter()
            .copied()
            .map(|owner| ckbtc::owner(owner, principal)),
    );

    let (address, accounts) = futures::join!(address, accounts);

    Ok(AccountDerivedAddress {
        pubkey,
        principal,
        btc: AccountDerivedBtcAddress {
            address: address?,
            accounts: accounts
                .into_iter()
                .map(|(owner, address)| (owner, address.map_or(None, Some)))
                .collect(),
        },
    })
}
