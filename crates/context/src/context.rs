use kube::{init_kube_client, kube_runtime};
use kube_runtime::Client as KubeClient;
use sqlx::PgPool;

use crate::postgres::create_pg_pool;

#[derive(Clone)]
pub struct KubeContext {
    #[allow(dead_code)]
    pub kube_client: KubeClient,
}

impl KubeContext {
    pub async fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            kube_client: init_kube_client().await?,
        })
    }
}

#[derive(Clone)]
pub struct PgContext {
    #[allow(dead_code)]
    pub pg_pool: PgPool,
}

impl PgContext {
    pub async fn new() -> Self {
        let pg_pool: PgPool = create_pg_pool().await;
        Self { pg_pool }
    }
}
