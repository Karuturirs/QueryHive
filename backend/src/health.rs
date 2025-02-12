use log::info;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
   pub status: String,
}


pub async fn liveness()  -> Json<HealthResponse> {
    info!("Liveness check hit");
    Json(HealthResponse { status: "OK".to_string() })
}

pub async fn readiness()  -> Json<HealthResponse> {
    info!("Readiness check hit");
    Json(HealthResponse { status: "OK".to_string() })
}

pub async fn health_check()  -> Json<HealthResponse> {
    info!("Health check hit");
    Json(HealthResponse { status: "OK".to_string() })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_liveness() {
        assert_eq!(liveness().await.status, "OK");
    }

    #[tokio::test]
    async fn test_readiness() {
        assert_eq!(readiness().await.status, "OK");
    }

    #[tokio::test]
    async fn test_health_check() {
        assert_eq!(health_check().await.status, "OK");
    }
}