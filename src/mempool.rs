use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::consensus::SaitoMessage;
use crate::crypto::{SaitoHash};
use crate::time::{create_timestamp};
use ::std::{sync::Arc, thread::sleep, time::Duration};
use tokio::sync::{broadcast, mpsc, RwLock};

#[derive(Clone, Debug)]
pub enum MempoolMessage {
    TestMessage,
    TryBundle,
}

/// The `Mempool` holds unprocessed blocks and transactions and is in control of
/// discerning when thenodeis allowed to create a block. It bundles the block and
/// sends it to the `Blockchain` to be added to the longest-chain. New `Block`s
/// received over the network are queued in the `Mempool` before being added to
/// the `Blockchain`
pub struct Mempool {
    broadcast_channel_sender: Option<broadcast::Sender<SaitoMessage>>,
    mempool_channel_sender: Option<mpsc::Sender<MempoolMessage>>,
    blocks: Vec<Block>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            broadcast_channel_sender: None,
            mempool_channel_sender: None,
	    blocks: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block) -> bool {

	let hash_to_insert = block.get_hash();

        for blk in &self.blocks {
            if blk.get_hash() == hash_to_insert {
        	return false;
            }
        }

	self.blocks.push(block);
        return true;

    }

    pub fn get_block(&mut self, hash: SaitoHash) -> Option<Block> {

	println!("Blockchain attempting to fetch block with hash: {:?}", hash);

	let mut block_found = false;
	let mut block_idx = 0;

	for i in 0..self.blocks.len() {
	    if self.blocks[0].get_hash() == hash {
	        block_idx = i;
		block_found = true;
		break;
	    }
        }

	if block_found {
	    let block = self.blocks.remove(block_idx);
	    return Some(block);
	}

	return None;

    }

    pub async fn bundle_block(&mut self, blockchain_lock: Arc<RwLock<Blockchain>>) {

        println!("Bundling a Block!");

        //
        // create the block and add it to our blocks vector
        //
        let blockchain = blockchain_lock.read().await;
        let previous_block_hash = blockchain.get_latest_block_hash();
        let previous_block_id = blockchain.get_latest_block_id();

        let mut block = self.generate_block_from_mempool_transactions(previous_block_id, previous_block_hash);

        block.set_hash();

        let block_hash = block.get_hash();

        self.add_block(block);

        if !self.broadcast_channel_sender.is_none() {
           self.broadcast_channel_sender.as_ref().unwrap()
                        .send(SaitoMessage::MempoolNewBlock { hash: block_hash })
                        .expect("error: Mempool - bundle_block Block message failed to send");
        }


    }

    pub fn can_bundle_block(&self, _blockchain_lock: Arc<RwLock<Blockchain>>) -> bool {
        true
    }

    pub fn generate_block_from_mempool_transactions(&mut self, previous_block_id : u64, previous_block_hash : SaitoHash) -> Block {

        let mut block = Block::new();
        block.set_id(previous_block_id);
        block.set_timestamp(create_timestamp());
        block.set_previous_block_hash(previous_block_hash);
	block.set_hash();

	return block;

    }


    pub fn set_broadcast_channel_sender(&mut self, bcs : broadcast::Sender<SaitoMessage>) {
      self.broadcast_channel_sender = Some(bcs);
    }
    pub fn set_mempool_channel_sender(&mut self, mcs : mpsc::Sender<MempoolMessage>) {
      self.mempool_channel_sender = Some(mcs);
    }

}




//
// This function is called on initialization to setup the sending
// and receiving channels for asynchronous loops or message checks
//
pub async fn run(
    mempool_lock: Arc<RwLock<Mempool>>,
    blockchain_lock: Arc<RwLock<Blockchain>>,
    broadcast_channel_sender: broadcast::Sender<SaitoMessage>,
    mut broadcast_channel_receiver: broadcast::Receiver<SaitoMessage>,
) -> crate::Result<()> {
    let (mempool_channel_sender, mut mempool_channel_receiver) = mpsc::channel(4);

    //
    // pass clones of our broadcast sender channels into Mempool so it 
    // can broadcast into the world as well...
    //
println!("about to write mempool to send channels in...");
    {
        let mut mempool = mempool_lock.write().await;
        mempool.set_broadcast_channel_sender(broadcast_channel_sender.clone());
        mempool.set_mempool_channel_sender(mempool_channel_sender.clone());
    }
println!("done with that, moving on...");

    //
    // loops to trigger messages
    //
    tokio::spawn(async move {
        loop {
            mempool_channel_sender
                .send(MempoolMessage::TryBundle)
                .await
                .expect("error: TryBundle message failed to send");
            sleep(Duration::from_millis(1000));
        }
    });


    loop {
        tokio::select! {

      	    //
	    // local messages
	    //
            Some(message) = mempool_channel_receiver.recv() => {
                match message {
		    //
		    // TryBundle
		    //
		    // the dominant local message is TryBundle, which triggrs
		    // periodic attempts to analyze the state of the mempool
		    // and make blocks if appropriate.
		    //
                    MempoolMessage::TryBundle => {
                        let mut mempool = mempool_lock.write().await;
                        if mempool.can_bundle_block(blockchain_lock.clone()) {
                            mempool.bundle_block(blockchain_lock.clone()).await;
                        }
                    },
		    _ => {}
                }
            }


      	    //
	    // system-wide messages
	    //
            Ok(message) = broadcast_channel_receiver.recv() => {
                match message {
		    //
		    // MempoolNewBlock
		    //
		    // triggered when the mempool produces a new block, we 
		    // hand off the block to the blockchain.
		    //
                    SaitoMessage::MempoolNewBlock { hash } => {
                        let mut mempool = mempool_lock.write().await;
                        let mut blockchain = blockchain_lock.write().await;
			let block = mempool.get_block(hash);
			if block.is_none() {
                            // bad block
                        } else {
                            blockchain.add_block(block.unwrap());
                        }
                    }
                    SaitoMessage::MempoolNewTransaction => {
                        let mut _mempool = mempool_lock.write().await;
                        println!("NEW TRANSACTION IN MEMPOOL");
                    },
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn mempool_new_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn mempool_can_bundle_block_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn mempool_bundle_block_test() {
        assert_eq!(true, true);
    }
}
