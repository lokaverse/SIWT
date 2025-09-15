use crate::types::*;

pub async fn all() -> impl Iterator<Item = (String, ByteBuf)> {
    Globals::all()
}

pub async fn get(key: impl AsRef<str>) -> Option<ByteBuf> {
    Globals::get(key)
}

pub async fn of(
    keys: impl Iterator<Item = impl AsRef<str>>,
) -> impl Iterator<Item = (String, ByteBuf)> {
    Globals::of(keys)
}

pub async fn keys() -> impl Iterator<Item = String> {
    Globals::keys()
}

pub async fn has(key: impl AsRef<str>) -> bool {
    Globals::has(key)
}

pub async fn contains(keys: impl Iterator<Item = impl AsRef<str>>) -> bool {
    Globals::contains(keys)
}

pub async fn store(key: impl AsRef<str>, value: impl AsRef<[u8]>) {
    Globals::store(key, value);
}

pub async fn stores(
    values: impl Iterator<Item = (impl AsRef<str>, impl AsRef<[u8]>)>,
) -> impl Iterator<Item = (String, ByteBuf)> {
    Globals::stores(values)
}

pub async fn remove(key: impl AsRef<str>) -> Option<ByteBuf> {
    Globals::remove(key)
}

pub async fn removes(
    keys: impl Iterator<Item = impl AsRef<str>>,
) -> impl Iterator<Item = (String, ByteBuf)> {
    Globals::removes(keys)
}
