use log::{info};

pub async fn liveness() -> &'static str {
    info!("Liveness check hit");
    "OK"
}

pub async fn readiness() -> &'static str {
    info!("Readiness check hit");
    "OK"
}

pub async fn health_check() -> &'static str {
    info!("Health check hit");
    "OK"
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_liveness() {
        assert_eq!(liveness().await, "OK");
    }

    #[tokio::test]
    async fn test_readiness() {
        assert_eq!(readiness().await, "OK");
    }

    #[tokio::test]
    async fn test_health_check() {
        assert_eq!(health_check().await, "OK");
    }
}