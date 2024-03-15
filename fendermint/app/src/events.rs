// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::BlockHeight;

pub struct ProposalProcessed<'a> {
    pub is_accepted: bool,
    pub block_height: BlockHeight,
    pub block_hash: &'a str,
    pub num_txs: usize,
    pub proposer: &'a str,
}

pub struct NewBlock {
    pub block_height: BlockHeight,
}