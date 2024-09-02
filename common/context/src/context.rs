use common::anyhow;
use kube::{init_kube_client, kube_runtime};
use kube_runtime::Client as KubeClient;

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
