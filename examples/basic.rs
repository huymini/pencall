use pencall::pencall::*;
use async_trait::async_trait;
use std::sync::Arc;
use chrono::Utc;

struct ConsoleProvider;

#[async_trait]
impl DeliveryProvider for ConsoleProvider {
    async fn deliver(&self, event: &ReleaseEvent) -> Result<(), PencallError> {
        println!("ReleaseEvent: {:?}", event);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), PencallError> {
    let store = Arc::new(Store::new());
    let provider = Arc::new(ConsoleProvider);
    let policy_check = Arc::new(|_req: &AllocationRequest| Ok(()));

    let pencall = Pencall::new(store.clone(), provider, policy_check);

    let req = AllocationRequest {
        id: "demo1".to_string(),
        requested_units: 16,
        start_time: Utc::now(),
        initial_release: Some(1),
        doubling_interval_seconds: 2,
        cap_units: Some(32),
    };

    pencall.register_allocation(req.clone()).await?;
    pencall.activate(&req.id).await?;
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    Ok(())
}
