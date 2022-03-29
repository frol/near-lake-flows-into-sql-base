use bigdecimal::BigDecimal;
use sqlx::Arguments;

#[derive(Debug, sqlx::FromRow)]
pub struct Chunk {
    pub included_in_block_hash: String,
    pub chunk_hash: String,
    pub shard_id: BigDecimal,
    pub signature: String,
    pub gas_limit: BigDecimal,
    pub gas_used: BigDecimal,
    pub author_account_id: String,
}

impl Chunk {
    // // TODO we actually don't need it, but I feel it could be useful somewhere
    // pub fn from_chunk_view(
    //     chunk_view: &near_indexer_primitives::IndexerChunkView,
    //     block_hash: &near_indexer_primitives::CryptoHash,
    // ) -> Self {
    //
    //     Self {
    //         included_in_block_hash: block_hash.to_string(),
    //         chunk_hash: chunk_view.header.chunk_hash.to_string(),
    //         shard_id: chunk_view.header.shard_id.into(),
    //         signature: chunk_view.header.signature.to_string(),
    //         gas_limit: chunk_view.header.gas_limit.into(),
    //         gas_used: chunk_view.header.gas_used.into(),
    //         author_account_id: chunk_view.author.to_string(),
    //     }
    // }

    pub fn add_to_args(
        chunk_view: &near_indexer_primitives::IndexerChunkView,
        block_hash: &near_indexer_primitives::CryptoHash,
        args: &mut sqlx::mysql::MySqlArguments,
    ) {
        args.add(block_hash.to_string());
        args.add(chunk_view.header.chunk_hash.to_string());
        args.add(BigDecimal::from(chunk_view.header.shard_id));
        args.add(chunk_view.header.signature.to_string());
        args.add(BigDecimal::from(chunk_view.header.gas_limit));
        args.add(BigDecimal::from(chunk_view.header.gas_used));
        args.add(chunk_view.author.to_string());
    }

    pub fn fields_count() -> usize {
        7
    }
}
