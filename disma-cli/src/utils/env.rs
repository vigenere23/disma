use std::env;

pub fn required_env(env_var: &str) -> String {
    env::var(env_var).unwrap_or_else(|_| panic!("Missing environment variable '{env_var}'."))
}
