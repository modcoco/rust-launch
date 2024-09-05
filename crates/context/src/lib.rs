mod postgres;

use std::time::Instant;

use chrono::{DateTime, NaiveDateTime, Utc};
use kube::{init_kube_client, kube_runtime};
use kube_runtime::Client as KubeClient;
use sqlx::PgPool;

use crate::postgres::create_pg_pool;

#[derive(Clone)]
pub struct AppContext {
    pub kube_client: KubeClient,
    pub pg_pool: PgPool,
    pub start_time: NaiveDateTime,
    pub running_time: Instant,
}

impl AppContext {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let kube_client = init_kube_client().await?;
        let pg_pool = create_pg_pool().await;
        let now: DateTime<Utc> = Utc::now() + chrono::Duration::hours(8);
        let start_time = now.naive_local();

        Ok(Self {
            kube_client,
            pg_pool,
            start_time,
            running_time: Instant::now(),
        })
    }
}
