use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use std::sync::{Arc, RwLock};

use crate::block::{Block, BlockHeader};
use crate::wallet::Wallet;
use crate::utxoset::UTXOSet;
use crate::storage::Storage;


/// BlockchainIndex syncs so that
/// every element in every vector references the same implicit
/// block, regardless of whether it is on the longest chain or
/// not.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BlockchainIndex {
    /// Vector of Block headers
    blocks:      Vec<BlockHeader>
}

impl BlockchainIndex {
    pub fn new() -> BlockchainIndex {
        return BlockchainIndex {
            blocks:      vec![],                 // blocks
        };
    }
}

///
/// Blockchain represent the state of the
/// blockchain itself, including the blocks that are on the
/// longest-chain as well as the material that is sitting off
/// the longest-chain but capable of being switched over.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Blockchain {

    index:          BlockchainIndex,
    bsh_lc_hmap:    HashMap<[u8; 32], u8>,
    bsh_bid_hmap:   HashMap<[u8; 32], u32>,

    lc_pos_set:     bool,
    lc_pos:         usize,

    genesis_ts:	    u64,
    genesis_bid:    u32,
    genesis_period: u32,

    last_bsh:			[u8; 32],
    last_bid:			u32,
    last_ts:			u64,
    last_bf:			f32,

    lowest_acceptable_ts:	u64,
    lowest_acceptable_bsh:	[u8; 32],
    lowest_acceptable_bid:	u32,

}

impl Blockchain {
    pub fn new() -> Blockchain {
        return Blockchain {
            index:         	       BlockchainIndex::new(),
            bsh_lc_hmap:   	       HashMap::new(),
            bsh_bid_hmap:  	       HashMap::new(),
            lc_pos_set:    	       false,
            lc_pos:        	       0,

            genesis_ts:	           0,
            genesis_bid:    	   0,
            genesis_period: 	   0,

            last_bsh:		       [0; 32],
            last_bid:		       0,
            last_ts:		       0,
            last_bf:		       0.0,

            lowest_acceptable_ts:  0,
            lowest_acceptable_bsh: [0; 32],
            lowest_acceptable_bid: 0,
        };
    }
    pub fn get_latest_block_header(&mut self) -> Option<BlockHeader> {
        return match !self.lc_pos_set {
            true => None,
            false => Some(self.index.blocks[self.lc_pos].clone())
        }
    }
    pub fn add_block(
        &mut self,
        blk: Block,
        wallet: &RwLock<Wallet>,
        utxoset: &mut UTXOSet,
    ) {
        // check block is superficially valid
        if blk.is_valid == 0 {
            println!("block is not valid - terminating add_block in blockchain...");
            return;
        }

        // ignore pre-genesis blocks
        if blk.body.ts < self.genesis_ts || blk.body.id < self.genesis_bid {
            // TODO - we ignore this restriction if we are loading from disk / forcing load
            println!("not adding block to blockchain -- block precedes genesis");
            return;
        }

        if blk.body.ts < self.lowest_acceptable_ts {
            self.lowest_acceptable_ts = blk.body.ts;
        }

        let pos: usize = self.index.blocks.len();
        self.bsh_bid_hmap.insert(blk.get_bsh(), blk.body.id);
        self.index.blocks.insert(pos, blk.header());

        // vars for determining the longest chain
        let i_am_the_longest_chain: u8  = 1;

        if i_am_the_longest_chain == 1 {
            self.last_bsh  = self.index.blocks[pos].bsh;
            self.last_ts   = self.index.blocks[pos].ts;
            self.last_bid  = self.index.blocks[pos].bid;
            self.lc_pos = pos;
            self.lc_pos_set = true;

            for tx in blk.body.txs.iter() {
                utxoset.spend_transaction(tx, blk.body.id);
                utxoset.insert_new_transaction(tx);
            }

            self.add_block_success(blk, wallet, 0, i_am_the_longest_chain, 0);
        }
    }

    fn add_block_success(
        &mut self,
        blk: Block,
        wallet: &RwLock<Wallet>,
        _pos: usize,
        i_am_the_longest_chain: u8,
        _force: u8
    ) {
        let publickey = wallet.read().unwrap().return_publickey();
        blk.body.txs
            .iter() 
            .for_each(|tx| {
                tx.get_from_slips()
                    .iter()
                    .filter(|slip| slip.return_add() == publickey)
                    .for_each(move |slip| {
                        if let Ok(mut wallet_guard) = wallet.write() {
                            wallet_guard.remove_slip(slip.clone());
                        }
                    });
                tx.get_to_slips()
                    .iter()
                    .filter(|slip| slip.return_add() == publickey)
                    .for_each(move |slip| {
                        if let Ok(mut wallet_guard) = wallet.write() {
                            wallet_guard.add_slip(slip.clone());
                        }
                    });
            });

        Storage::write_block_to_disk(blk);
        println!("Adding block: {:?}", self.last_bsh);
    }
    // fn get_latest() -> Block {
    //     Block {}
    // }
    // fn get_block_by_id(id: u32) -> Block {
    //     Block {}
    // }
    // fn get_block_by_hash(hash: &str) -> Block {
    //     Block {}
    // }
    // fn add_block(block: Block, parentId: i64) {
    //   self.blocks.push(block);
    // }
    // fn get_block_parent(block: Block) -> Block {
    //     Block {}
    // }
    // fn wind_chain() -> bool {}
    // fn unwind_chain() -> bool {}
}

// Event: rollBackBlock(Block block)
// Event: rollForwardBlock(Block block)

#[cfg(test)]
mod test {
    #[test]
    fn test_new() {
        assert!(false);
    }
    #[test]
    fn test_get_block_by_id() {
        assert!(false);
    }
    #[test]
    fn test_get_block_by_hash() {
        assert!(false);
    }
    #[test]
    fn test_add_block() {
        assert!(false);
    }
    #[test]
    fn test_get_block_parent() {
        assert!(false);
    }
}
