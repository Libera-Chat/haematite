#![allow(clippy::module_name_repetitions)]

pub trait NoneOr<T> {
    fn none_or<E>(&mut self, error: E) -> Result<(), E>;
}

impl<T> NoneOr<T> for Option<T> {
    fn none_or<E>(&mut self, error: E) -> Result<(), E> {
        match self.take() {
            Some(_) => Err(error),
            None => Ok(()),
        }
    }
}

pub trait FalseOr {
    fn false_or<E>(&self, error: E) -> Result<(), E>;
}

impl FalseOr for bool {
    fn false_or<E>(&self, error: E) -> Result<(), E> {
        if *self {
            Err(error)
        } else {
            Ok(())
        }
    }
}

pub trait TrueOr {
    fn true_or<E>(&self, error: E) -> Result<(), E>;
}

impl TrueOr for bool {
    fn true_or<E>(&self, error: E) -> Result<(), E> {
        if *self {
            Ok(())
        } else {
            Err(error)
        }
    }
}
