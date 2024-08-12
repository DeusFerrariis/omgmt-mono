use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;

use crate::product::{Amount, ProductEvent};

pub struct SurrealProvider;

#[derive(Serialize, Deserialize)]
pub struct EventEnvelope<T>
where
    T: Serialize,
{
    name: String,
    details: T,
    timestamp: chrono::DateTime<chrono::Utc>,
}

fn wrap_event<T: Serialize>(name: String, details: T) -> EventEnvelope<T> {
    EventEnvelope {
        name,
        details,
        timestamp: chrono::Utc::now(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct DatedEvent<T>
where
    T: Serialize,
{
    details: T,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T: Serialize> DatedEvent<T> {
    fn from_envelope(envelope: EventEnvelope<T>) -> Self {
        DatedEvent {
            details: envelope.details,
            timestamp: envelope.timestamp,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    id: surrealdb::sql::Thing,
}

impl SurrealProvider {
    async fn write_event<T: Serialize, C: surrealdb::Connection>(
        conn: surrealdb::Surreal<C>,
        event: EventEnvelope<T>,
    ) -> Result<String, SkipError> {
        let result: Vec<Record> = conn
            .create(format!("event_{}", event.name))
            .content(DatedEvent::from_envelope(event))
            .await?;

        let Some(record) = result.get(0) else {
            return Err(SkipError);
        };

        Ok(record.id.id.to_string())
    }

    async fn create_product<C: surrealdb::Connection>(
        conn: surrealdb::Surreal<C>,
        sku: crate::product::SKU,
        retail: crate::product::Amount,
    ) -> Result<String, SkipError> {
        // Assert sku does not exist
        let mut result = conn
            .query("SELECT * FROM type::table($table) WHERE sku=$sku")
            .bind(("table", "event_product-created"))
            .bind(("sku", &sku))
            .await?;

        let products: Vec<DatedEvent<ProductEvent>> = result.take(0)?;
        if products.len() != 0 {
            return Err(SkipError);
        }

        let product = crate::product::ProductEvent::Created { sku, retail };
        let envelope = wrap_event("product-created".to_string(), product);
        SurrealProvider::write_event(conn, envelope).await
    }
}

pub struct SkipError;

impl From<surrealdb::Error> for SkipError {
    fn from(value: surrealdb::Error) -> Self {
        SkipError
    }
}
