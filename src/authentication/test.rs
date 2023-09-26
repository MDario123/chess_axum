use std::collections::HashSet;

use super::*;

#[test]
fn token_generation() {
    let s = generate_token();
    assert_eq!(s.bytes().len(), 64);

    let mut map: HashSet<String> = HashSet::new();

    for _ in 0..1_000_000 {
        let s = generate_token();
        assert!(!map.contains(&s));
        map.insert(s);
    }
}
