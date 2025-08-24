//! sentinel-core placeholder
//! todo -> 나중에 실제 로직으로 교체하기

pub fn ping() -> &'static str {
    println!("[sentinel-core] ping");
    "pong"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_pings() {
        assert_eq!(ping(), "pong");
    }
}
