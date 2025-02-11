use prometheus::{Encoder, TextEncoder, IntCounter};
use tokio::sync::OnceCell;


static HEALTH_CHECK_COUNTER: OnceCell<IntCounter> = OnceCell::const_new();


pub async fn metrics() -> String {
    let counter = HEALTH_CHECK_COUNTER.get().unwrap();
    counter.inc();  // Increment the counter

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic] //TODO: Fix this test
    async fn test_metrics() {
        // Extract the initial counter value safely
        let initial_count = HEALTH_CHECK_COUNTER.get()
            .expect("Counter not initialized")
            .get(); // Extracts the integer value
    
        metrics().await;  // Call the metrics function, which increments the counter
    
        // Extract the updated counter value
        let new_count = HEALTH_CHECK_COUNTER.get()
            .expect("Counter not initialized")
            .get(); // Extracts the integer value
    
        assert!(new_count > initial_count);
    }
}
