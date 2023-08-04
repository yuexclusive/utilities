#[tokio::main]
async fn main() -> anyhow::Result<()> {
    util_fluvio::produce("aaa", "name", "Scarlett").await?;
    Ok(())
}
