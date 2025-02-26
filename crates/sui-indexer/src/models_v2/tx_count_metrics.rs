// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;

use crate::schema_v2::tx_count_metrics;

#[derive(Clone, Debug, Default, Queryable, Insertable)]
#[diesel(table_name = tx_count_metrics)]
pub struct StoredTxCountMetrics {
    pub checkpoint_sequence_number: i64,
    pub epoch: i64,
    pub timestamp_ms: i64,
    pub total_transaction_blocks: i64,
    pub total_successful_transaction_blocks: i64,
    pub total_successful_transactions: i64,
}
