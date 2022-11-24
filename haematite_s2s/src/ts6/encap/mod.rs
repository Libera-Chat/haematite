mod su;

use haematite_models::irc::network::Network;
use std::collections::HashMap;

use crate::handler::{ArgumentCountResolver, Error, LineHandlerResolution, LineHandlerResolver};
use crate::line::Line;

pub(super) struct HandlerResolver {
    handler_resolvers: HashMap<&'static [u8], LineHandlerResolution>,
}

impl LineHandlerResolver for HandlerResolver {
    fn resolve(
        &mut self,
        _network: &Network,
        line: &Line,
    ) -> Result<Option<&mut LineHandlerResolution>, Error> {
        if let Some(resolver) = self.handler_resolvers.get_mut(line.args[1].as_slice()) {
            Ok(Some(resolver))
        } else {
            Ok(None)
        }
    }
}

pub(super) struct Handler {}

impl Handler {
    pub fn resolver() -> impl LineHandlerResolver {
        ArgumentCountResolver::from_resolver(
            2,
            usize::MAX,
            HandlerResolver {
                handler_resolvers: HashMap::from([(
                    b"SU".as_slice(),
                    LineHandlerResolution::SeeOther(Box::new(su::Handler::resolver())),
                )]),
            },
        )
    }
}
