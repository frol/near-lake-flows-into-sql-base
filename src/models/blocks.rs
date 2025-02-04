use std::str::FromStr;

use bigdecimal::BigDecimal;
use sqlx::Arguments;

use crate::models::FieldCount;

#[derive(Debug, sqlx::FromRow, FieldCount)]
pub struct Block {
    pub block_height: BigDecimal,
    pub block_hash: String,
    pub prev_block_hash: String,
    pub block_timestamp: BigDecimal,
    pub total_supply: BigDecimal,
    pub gas_price: BigDecimal,
    pub author_account_id: String,
}

impl Block {
    pub fn from_block_view(block_view: &near_indexer_primitives::views::BlockView) -> Self {
        Self {
            block_height: block_view.header.height.into(),
            block_hash: block_view.header.hash.to_string(),
            prev_block_hash: block_view.header.prev_hash.to_string(),
            block_timestamp: block_view.header.timestamp.into(),
            total_supply: BigDecimal::from_str(block_view.header.total_supply.to_string().as_str())
                .expect("`total_supply` expected to be u128"),
            gas_price: BigDecimal::from_str(block_view.header.gas_price.to_string().as_str())
                .expect("`gas_price` expected to be u128"),
            author_account_id: block_view.author.to_string(),
        }
    }

    pub fn add_to_args(&self, args: &mut sqlx::mysql::MySqlArguments) {
        args.add(&self.block_height);
        args.add(&self.block_hash);
        args.add(&self.prev_block_hash);
        args.add(&self.block_timestamp);
        args.add(&self.total_supply);
        args.add(&self.gas_price);
        args.add(&self.author_account_id);
    }

    pub fn get_query(blocks_count: usize) -> anyhow::Result<String> {
        crate::models::create_query_with_placeholders(
            "INSERT IGNORE INTO blocks VALUES",
            blocks_count,
            Block::field_count(),
        )
    }
}
