use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub database: DatabaseConfig,
    pub verification_expiration: i64,
    pub auth: AuthConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
            verification_expiration: option_env!("VERIFICATION_EXPIRATION")
                .unwrap_or((60 * 5).to_string().as_str())
                .parse()
                .expect("VERIFYCATION_EXPIRATION must be a number"),
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}
