pub use fluvio::Offset;
use fluvio::{
    dataplane::record::{ConsumerRecord, RecordData, RecordKey},
    PartitionSelectionStrategy, ProduceOutput,
};
use std::collections::HashMap;
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot::{self, Receiver},
};

use std::hash::Hash;
use tokio_stream::StreamExt;

pub async fn produce<S, K, V>(topic: S, key: K, value: V) -> anyhow::Result<ProduceOutput>
where
    S: Into<String>,
    K: Into<RecordKey>,
    V: Into<RecordData>,
{
    let conn = fluvio::Fluvio::connect().await?;
    let producer = conn.topic_producer(topic).await?;
    let res = producer.send(key, value).await?;
    producer.flush().await?;
    Ok(res)
}

pub async fn produce_batch<S, K, V>(
    topic: S,
    kvs: Vec<(K, V)>,
) -> anyhow::Result<HashMap<K, ProduceOutput>>
where
    S: Into<String>,
    K: Into<RecordKey> + Eq + PartialEq + Hash + Clone,
    V: Into<RecordData>,
{
    let conn = fluvio::Fluvio::connect().await?;
    let producer = conn.topic_producer(topic).await?;
    let mut hm = HashMap::new();

    for (key, value) in kvs.into_iter() {
        let res = producer.send(key.clone(), value).await?;
        hm.insert(key, res);
    }
    producer.flush().await?;
    Ok(hm)
}

pub async fn consume<S, F>(
    topics_with_partition: Vec<(S, u32)>,
    offset: Offset,
    mut callback: F,
) -> anyhow::Result<(Sender<()>, Receiver<()>)>
where
    S: Into<String>,
    F: FnMut(ConsumerRecord) + Send + 'static,
{
    let conn = fluvio::Fluvio::connect().await?;
    let topics_with_partition = topics_with_partition
        .into_iter()
        .map(|(k, v)| (k.into(), v))
        .collect();
    let consumer = conn
        .consumer(PartitionSelectionStrategy::Multiple(topics_with_partition))
        .await?;

    let mut stream = consumer.stream(offset).await?;
    let (sender, mut receiver) = mpsc::channel::<()>(1);
    let (close_sender, close_receiver) = oneshot::channel::<()>();

    tokio::spawn(async move {
        'l: loop {
            tokio::select! {
                Some(Ok(v)) = stream.next() =>{
                    callback(v)
                }
                Some(_) = receiver.recv() => {
                    break 'l
                }
            }
        }
        close_sender.send(()).unwrap();
    });

    Ok((sender, close_receiver))
}
