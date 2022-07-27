pub mod user;

use serde::Serialize;

pub trait Permission: Default {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Serialize)]
pub struct With<V, P: Permission> {
    pub value: V,
    pub permission: P,
}

impl<V, P: Permission> From<V> for With<V, P> {
    fn from(value: V) -> With<V, P> {
        With {
            value,
            permission: P::new(),
        }
    }
}

impl<V, P: Permission> With<V, P> {
    pub fn new(value: V, permission: P) -> Self {
        Self { value, permission }
    }
}

macro_rules! define_permission {
    ($typename:ident) => {
        #[derive(Default, serde::Serialize)]
        pub struct $typename;
        impl crate::permission::Permission for $typename {}
    };
}

pub(crate) use define_permission;
