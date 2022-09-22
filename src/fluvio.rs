// use fluvio::dataplane::record::RecordData;
// use fluvio::RecordKey;

// pub const NULL_KEY: RecordKey = RecordKey::NULL;

// pub async fn produce<T, K, V>(topic: T, key: K, data: V) -> Result<(), fluvio::FluvioError>
// where
//     T: Into<String>,
//     K: Into<RecordKey>,
//     V: Into<RecordData>,
// {
//     let producer = fluvio::producer(topic).await?;

//     let value = data;
//     producer.send(key, value).await?;
//     producer.flush().await?;

//     Ok(())
// }

// async fn consume<T>(topic: T, partition: i32) -> Result<(), fluvio::FluvioError>
// where
//     T: Into<String>,
// {
//     use futures_lite::StreamExt;

//     let consumer = fluvio::consumer(topic, 0).await?;
//     let mut stream = consumer.stream(fluvio::Offset::beginning()).await?;

//     while let Some(Ok(record)) = stream.next().await {
//         let string = String::from_utf8_lossy(record.value());
//         println!("{}", string);
//     }
//     Ok(())
// }
