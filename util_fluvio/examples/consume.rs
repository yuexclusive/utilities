use util_fluvio::Offset;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (close_sender, close_done_receiver) =
        util_fluvio::consume(vec![("aaa", 0)], Offset::end(), |v| {
            println!("{}", v.record.value)
        })
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

    close_sender.send(()).await?;
    close_done_receiver.await?;
    println!("{}", "closed");
    Ok(())
}
