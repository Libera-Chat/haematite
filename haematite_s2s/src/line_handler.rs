use haematite_models::irc::network::Network;

use crate::handler::{Error, Outcome};
use crate::line::Line;

pub enum Resolution {
    SeeOther(Box<dyn HandlerResolver>),
    Handler(Box<dyn Handler>),
}

pub trait Handler {
    fn handle(&mut self, network: &Network, line: &Line) -> Result<Outcome, Error>;
}

pub trait HandlerResolver {
    fn resolve(&mut self, network: &Network, line: &Line)
        -> Result<Option<&mut Resolution>, Error>;
}

pub struct NoOpResolver {
    resolution: Resolution,
}

impl NoOpResolver {
    pub fn new<T: Handler + 'static>(handler: T) -> Self {
        Self {
            resolution: Resolution::Handler(Box::new(handler)),
        }
    }
}

impl HandlerResolver for NoOpResolver {
    fn resolve(
        &mut self,
        _network: &Network,
        _line: &Line,
    ) -> Result<Option<&mut Resolution>, Error> {
        Ok(Some(&mut self.resolution))
    }
}

pub struct ArgumentCountResolver {
    resolution: Resolution,
    minimum: usize,
    maximum: usize,
}

impl ArgumentCountResolver {
    pub fn from_handler<T: Handler + 'static>(minimum: usize, maximum: usize, handler: T) -> Self {
        Self {
            resolution: Resolution::Handler(Box::new(handler)),
            minimum,
            maximum,
        }
    }

    pub fn from_resolver<T: HandlerResolver + 'static>(
        minimum: usize,
        maximum: usize,
        resolver: T,
    ) -> Self {
        Self {
            resolution: Resolution::SeeOther(Box::new(resolver)),
            minimum,
            maximum,
        }
    }
}

impl HandlerResolver for ArgumentCountResolver {
    fn resolve(
        &mut self,
        _network: &Network,
        line: &Line,
    ) -> Result<Option<&mut Resolution>, Error> {
        let actual = line.args.len();
        if actual < self.minimum {
            Err(Error::InsufficientArguments {
                expected: self.minimum,
                actual,
            })
        } else if actual > self.maximum {
            Err(Error::ExcessArguments {
                expected: self.maximum,
                actual,
            })
        } else {
            Ok(Some(&mut self.resolution))
        }
    }
}
