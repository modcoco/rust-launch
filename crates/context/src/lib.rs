mod postgres;

use kube::{init_kube_client, kube_runtime};
use kube_runtime::Client as KubeClient;
use sqlx::PgPool;

use crate::postgres::create_pg_pool;

#[derive(Clone)]
pub struct AppContext {
    pub kube_client: KubeClient,
    pub pg_pool: PgPool,
}

impl AppContext {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let kube_client = init_kube_client().await?;
        let pg_pool = create_pg_pool().await;

        Ok(Self {
            kube_client,
            pg_pool,
        })
    }
}
