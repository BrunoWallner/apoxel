use super::Handle;

use std::time::Duration;
use tokio::{task, time};

pub fn init_load_requester(
    handle: Handle,
) {
    tokio::spawn(async move {
        let forever = task::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(1000));
    
            loop {
                interval.tick().await;
                handle.load().await;
            }
        });
    
        forever.await.unwrap();
    });
}