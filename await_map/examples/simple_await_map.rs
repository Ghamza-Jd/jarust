use await_map::AwaitMap;
use std::{sync::Arc, time::Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let await_map = Arc::new(AwaitMap::new());

    tokio::spawn({
        let map = await_map.clone();
        async move {
            map.insert("test", 1).await;
        }
    });

    tokio::spawn({
        let map = await_map.clone();
        async move {
            tokio::time::sleep(Duration::from_secs(5)).await;
            let res = map.get("test").await;
            dbg!(res);
        }
    });

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("tick");
    }
}
