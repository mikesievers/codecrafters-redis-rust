use crate::Resp;

pub fn cmd_ping() -> Resp {
    Resp::Simple("PONG".into())
}
