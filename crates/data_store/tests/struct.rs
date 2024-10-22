use std::collections::HashMap;

#[tokio::test]
async fn test_key_type() -> anyhow::Result<()> {
    type KeyType = (u64, u64, u64, String, String, i32, i32);
    let summary_map: HashMap<KeyType, i32> = HashMap::new();
    println!("{:?}", summary_map);
    Ok(())
}
