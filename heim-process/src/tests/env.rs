use crate::{EnvOs};

#[test]
fn parse_empty() {
    let env = EnvOs::from_bytes(b"");

    assert_eq!(0, env.count());
}
