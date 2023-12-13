use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use std::future::Future;
use tokio_util::sync::CancellationToken;

pub async fn consume_messages<F, Fut>(
    bootstrap_servers: String,
    topic: String,
    group_id: String,
    shutdown_cts: CancellationToken,
    process_message: F,
) -> Result<(), KafkaError>
where
    F: Fn(Vec<u8>) -> Fut,
    Fut: Future<Output = ()>,
{
    let mut con = Consumer::from_hosts(
        bootstrap_servers
            .split(',')
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    )
    .with_topic(topic)
    .with_group(group_id)
    .with_fallback_offset(FetchOffset::Earliest)
    .with_offset_storage(Some(GroupOffsetStorage::Kafka))
    .create()?;

    loop {
        if shutdown_cts.is_cancelled() {
            break;
        }
        let mss = con.poll()?;
        if mss.is_empty() {
            continue;
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                process_message(m.value.to_vec()).await;
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }

    Ok(())
}
