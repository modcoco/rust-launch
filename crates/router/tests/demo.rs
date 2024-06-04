pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use common::anyhow::Result;
    use common::reqwest;
    use common::tokio;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_tokio() -> Result<()> {
        let client = reqwest::Client::new();
        let res = client
            .get("http://127.0.0.1:8080/api/v1/test")
            .send()
            .await?;

        println!("resp: {}", res.status());

        let body = res.text().await?;

        println!("body: {}", body);
        Ok(())
    }
}
