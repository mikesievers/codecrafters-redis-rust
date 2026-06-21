use crate::Resp;

pub fn cmd_echo(args: &[Resp]) -> Resp {
    args.first().cloned().unwrap_or(Resp::Simple("".into()))
}
