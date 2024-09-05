use crate::models::metadata::Metadata;
use crate::{error::Result, models::metadata::MetadataBase};
use sqlx::{QueryBuilder, Sqlite, Transaction};

pub async fn add_metadata(
    data: Vec<Metadata>,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<()> {
    if !data.is_empty() {
        let mut metadata_query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("INSERT INTO metadata (id, listener_id, name, data) ");

        metadata_query_builder.push_values(data, |mut b, metadata| {
            b.push_bind(metadata.id)
                .push_bind(metadata.listener_id)
                .push_bind(metadata.base.name)
                .push_bind(metadata.base.data);
        });

        metadata_query_builder
            .build()
            .execute(&mut **transaction)
            .await?;
    }

    Ok(())
}

pub async fn create_metadata(
    listener_id: i64,
    data: Vec<MetadataBase>,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<()> {
    if !data.is_empty() {
        let mut metadata_query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("INSERT INTO metadata (listener_id, name, data) ");

        metadata_query_builder.push_values(&data, |mut b, meta| {
            b.push_bind(listener_id)
                .push_bind(&meta.name)
                .push_bind(&meta.data);
        });

        metadata_query_builder
            .build()
            .execute(&mut **transaction)
            .await?;
    }

    Ok(())
}
