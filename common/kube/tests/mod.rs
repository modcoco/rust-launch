#[cfg(test)]
mod tests {
    use common::anyhow;
    use common::axum::http::HeaderMap;
    use common::reqwest::blocking::Client;
    use common::reqwest::header::AUTHORIZATION;
    use common::reqwest::Certificate;
    use common::{tokio, tracing, url_https_builder};
    use k8s_openapi::api::core::v1::Namespace;
    use k8s_openapi::api::core::v1::Pod;
    use kube::ServiceAccountToken;
    use kube_runtime::{api::ListParams, Api, Client as KubeClient, Config};

    #[test]
    fn str_trimmed() {
        let str = "nvidia.com";
        let trimmed_str = str.trim_end_matches(".com");
        println!("{}", trimmed_str);
    }

    #[test]
    fn test_env() {
        let ps = ServiceAccountToken::new();
        println!("{:?}", ps)
    }

    #[test]
    fn rquest_tls() -> Result<(), anyhow::Error> {
        logger::logger_trace::init_logger();

        let sat = ServiceAccountToken::new();
        let kubernetes_token = sat.token;
        let kubernetes_cert = Certificate::from_pem(&sat.cacrt)?;

        let client = Client::builder()
            .use_rustls_tls()
            .add_root_certificate(kubernetes_cert)
            .build()?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", kubernetes_token).parse()?,
        );

        let url = url_https_builder(&sat.kube_host, &sat.kube_port, Some("/version"));
        let response = client.get(url).headers(headers).send()?;

        tracing::info!("{}", response.status());
        tracing::info!("{}", response.text()?);
        Ok(())
    }

    #[test]
    fn rquest_pods() -> Result<(), anyhow::Error> {
        logger::logger_trace::init_logger();

        let sat = ServiceAccountToken::new();
        let kubernetes_token = sat.token;
        let kubernetes_cert = Certificate::from_pem(&sat.cacrt)?;

        let client = Client::builder()
            .use_rustls_tls()
            .add_root_certificate(kubernetes_cert)
            .build()?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", kubernetes_token).parse()?,
        );

        let url = url_https_builder(
            &sat.kube_host,
            &sat.kube_port,
            Some("/api/v1/namespaces/default/pods"),
        );
        let response = client.get(url).headers(headers).send()?;

        tracing::info!("{}", response.status());
        tracing::info!("{}", response.text()?);
        Ok(())
    }

    #[tokio::test]
    pub async fn rquest_pods_kube() -> Result<(), anyhow::Error> {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider");

        let config = Config::infer().await?;
        // let config = Config::incluster()?;
        let client = KubeClient::try_from(config)?;

        let pods: Api<Pod> = Api::namespaced(client.clone(), "default");

        let lp = ListParams::default();
        let pods = pods.list(&lp).await?;

        for p in pods {
            println!("name {:?}", p.metadata.name);
        }

        let namespaces: Api<Namespace> = Api::all(client);

        let lp = ListParams::default();
        let ns_list = namespaces.list(&lp).await?;

        for ns in ns_list.items {
            let ns_name = ns.metadata.name.as_deref().unwrap_or("<unknown>");
            println!("Namespace name: {}", ns_name);
        }

        Ok(())
    }
}
