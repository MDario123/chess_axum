use rand::distributions::{Alphanumeric, DistString};

#[cfg(test)]
mod test;

pub(crate) fn is_pass_equivalent(a: &str, b: &str) -> bool {
    a == b
}

pub(crate) fn generate_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 64)
}
