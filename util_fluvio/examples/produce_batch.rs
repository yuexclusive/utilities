#[tokio::main]
async fn main() -> anyhow::Result<()> {
    util_fluvio::produce_batch("aaa", vec![("name", "Scarlett"), ("name", "Audrey")]).await?;
    Ok(())
}
