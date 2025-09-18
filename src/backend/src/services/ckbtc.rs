use crate::canisters::ckbtc::minter::GetBtcAddressArg;
use crate::canisters::ckbtc::minter::Service;
use crate::types::{ByteBuf, Principal};

fn minter() -> Result<Service, String> {
    let id = match option_env!("DFX_NETWORK") {
        Some("ic") => "mqygn-kiaaa-aaaar-qaadq-cai",
        Some("playground") => "ml52i-qqaaa-aaaar-qaaba-cai",
        _ => return Err("Cannot access ckbtc minter locally".to_string()),
    };

    Ok(Service(Principal::from_text(id).unwrap()))
}

pub fn sub_account(principal: Principal) -> [u8; 32] {
    let bytes = principal.as_slice();
    let mut buffer = [0u8; 32];

    for i in 0..32 {
        buffer[31 - i] = if i < bytes.len() {
            bytes[bytes.len() - 1 - i]
        } else {
            0
        };
    }

    buffer
}

pub async fn address(owner: Principal, account: Option<ByteBuf>) -> Result<String, String> {
    let payload = GetBtcAddressArg {
        owner: Some(owner),
        subaccount: account,
    };

    match minter()?.get_btc_address(payload).await {
        Ok((address,)) => Ok(address),
        Err((_, e)) => Err(e),
    }
}

pub async fn owned(principal: Principal) -> Result<String, String> {
    address(principal, None).await
}

pub async fn owner(owner: Principal, principal: Principal) -> (Principal, Result<String, String>) {
    (
        owner,
        address(owner, Some(ByteBuf::from(sub_account(principal)))).await,
    )
}
