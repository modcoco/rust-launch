use arc_swap::ArcSwap;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;
use std::sync::Arc;

static CONFIG: Lazy<ArcSwap<Config>> = Lazy::new(|| ArcSwap::from(Arc::new(Config::from_env())));

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app_env: String,
    pub database_url: String,
    pub kubernetes_ca_cert_path: Option<String>,
    pub kubernetes_namespace_path: Option<String>,
    pub kubernetes_token_path: Option<String>,
    pub kubernetes_service_host: Option<String>,
    pub kubernetes_service_port: Option<String>,
}

impl Config {
    fn from_env() -> Self {
        dotenv().ok();
        Self {
            app_env: env::var("APP_ENV").unwrap_or_else(|_| "prod".to_string()),
            database_url: env::var("DATABASE_URL").unwrap_or_default(),
            kubernetes_ca_cert_path: env::var("KUBERNETES_CA_CERT_PATH").ok(),
            kubernetes_namespace_path: env::var("KUBERNETES_NAMESPACE_PATH").ok(),
            kubernetes_token_path: env::var("KUBERNETES_TOKEN_PATH").ok(),
            kubernetes_service_host: env::var("KUBERNETES_SERVICE_HOST").ok(),
            kubernetes_service_port: env::var("KUBERNETES_SERVICE_PORT").ok(),
        }
    }
}

pub fn get_config() -> Arc<Config> {
    CONFIG.load().clone()
}
