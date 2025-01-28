use crate::core::types::Closure;
use crate::modules::seth::catfish::*;
use crate::modules::seth::qrcode::*;
use crate::modules::seth::server::*;

pub fn api() -> Closure {
    vec![
        ("qrcode", gen_qrcode_from_argsv),
        ("server.eviltwin", evil_server),
    ]
}
