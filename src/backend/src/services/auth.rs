use crate::payloads::{DelegationPayload, LoginPayload, PreparePayload};
use crate::responses::{Login, Prepared, SignedDelegation};
use crate::types::{ByteBuf, Delegation, Message, Messages, Setting};

pub async fn prepare(payload: PreparePayload) -> Result<Prepared, String> {
    payload.validate()?;

    let message: Message = payload.into();

    Messages::put(&message);

    ic_cdk::println!("{message}");

    Ok(Prepared::from(message))
}

pub async fn login(payload: LoginPayload) -> Result<Login, String> {
    let message = Messages::delete(payload.hash()).ok_or(format!("Hash not found or expired"))?;
    let setting = Setting::get();
    let delegation = Delegation::new(
        message.user(),
        message.session(),
        setting.expiration(),
        message.canisters().copied(),
    )?;

    Ok(Login {
        expiration: delegation.expiration(),
        expired: delegation.expiration().into(),
        canisters: delegation.targets().copied().collect(),
        hash: delegation.store(),
    })
}

pub async fn delegation(
    certificate: ByteBuf,
    payload: DelegationPayload,
) -> Result<SignedDelegation, String> {
    payload.validate()?;

    payload.into_delegation()?.sign(certificate)
}
