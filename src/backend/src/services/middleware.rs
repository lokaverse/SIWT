use crate::payloads::{
    MiddlewareDelegationPayload, MiddlewareLoginPayload, MiddlewareStoreSignaturePayload,
};
use crate::responses::{Login, MiddlewareDelegated, MiddlewareSignedDelegation};
use crate::types::{
    data_certificate, Delegation, Hash, Messages, Middleware, Middlewares, Setting,
};

pub async fn get(hash: Hash) -> Option<Middleware> {
    Middlewares::borrow(|middlewares| middlewares.get(&hash))
}

pub async fn store(payload: MiddlewareStoreSignaturePayload) {
    Middlewares::modify(&payload.hash, |m| m.sign(&payload.signature));
}

pub async fn login(payload: MiddlewareLoginPayload) -> Result<Login, String> {
    let message = Messages::delete(payload.hash()).ok_or(format!("Hash not found or expired"))?;
    let setting = Setting::get();
    let delegation = Delegation::new(
        message.user(),
        message.session(),
        setting.expiration(),
        setting.canisters().copied(),
    )?;

    Ok(Login {
        expiration: delegation.expiration(),
        expired: delegation.expiration().into(),
        canisters: delegation.targets().cloned().collect(),
        hash: delegation.store_with_middleware(payload.middleware()),
    })
}

pub async fn delegation(
    payload: MiddlewareDelegationPayload,
) -> Result<MiddlewareSignedDelegation, String> {
    payload.validate()?;

    let session = payload
        .into_session_delegation()?
        .sign(data_certificate().unwrap())?;

    let middleware = payload
        .into_middleware_delegation()?
        .sign(data_certificate().unwrap())?;

    Ok(MiddlewareSignedDelegation {
        session: MiddlewareDelegated {
            pubkey: session.delegation.pubkey,
            signature: session.signature,
        },
        middleware: MiddlewareDelegated {
            pubkey: middleware.delegation.pubkey,
            signature: middleware.signature,
        },
        expiration: session.delegation.expiration,
        canisters: session.delegation.targets,
        pubkey: session.pubkey,
    })
}
