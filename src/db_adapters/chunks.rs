use crate::models;
use itertools::Itertools;

pub(crate) async fn store_chunks(
    pool: &sqlx::Pool<sqlx::Postgres>,
    shards: &[near_indexer_primitives::IndexerShard],
    block_hash: &near_indexer_primitives::CryptoHash,
) -> anyhow::Result<()> {
    // Processing by parts to avoid huge bulk insert statements
    for chunks_part in &shards
        .iter()
        .filter_map(|shard| shard.chunk.as_ref())
        .chunks(crate::db_adapters::CHUNK_SIZE_FOR_BATCH_INSERT)
    {
        let mut args = sqlx::postgres::PgArguments::default();
        let mut chunks_count = 0;

        chunks_part.for_each(|chunk| {
            models::chunks::Chunk::from_chunk_view(chunk, block_hash).add_to_args(&mut args);
            chunks_count += 1;
        });

        let query = models::chunks::Chunk::get_query(chunks_count)?;
        sqlx::query_with(&query, args).execute(pool).await?;
    }

    Ok(())
}
