/// A Peer. i.e. another node in the network.
use crate::block::{Block, BlockType};
use crate::blockchain::{Blockchain, GENESIS_PERIOD};
use crate::consensus::SaitoMessage;
use crate::crypto::{hash, verify, SaitoHash, SaitoPublicKey};
use crate::hop::Hop;
use crate::mempool::Mempool;
use crate::network::{
    Network, CHALLENGE_EXPIRATION_TIME, CHALLENGE_SIZE, INBOUND_PEER_CONNECTIONS_GLOBAL,
    OUTBOUND_PEER_CONNECTIONS_GLOBAL, PEERS_DB_GLOBAL, PEERS_REQUEST_RESPONSES_GLOBAL,
    PEERS_REQUEST_WAKERS_GLOBAL,
};
use crate::networking::message_types::handshake_challenge::HandshakeChallenge;
use crate::networking::message_types::request_block_message::RequestBlockMessage;
use crate::networking::message_types::request_blockchain_message::RequestBlockchainMessage;
use crate::networking::message_types::send_block_head_message::SendBlockHeadMessage;
use crate::networking::message_types::send_blockchain_message::{
    SendBlockchainBlockData, SendBlockchainMessage, SyncType,
};
use crate::time::create_timestamp;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
use async_recursion::async_recursion;
use futures::stream::SplitSink;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{error, info};
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use crate::networking::api_message::APIMessage;
use futures::{Future, FutureExt, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};





/// PeerType indicates whether this peer was added by us as a desired outbound
/// connection or whether it came to us via an inbound connection.
#[derive(Serialize, Deserialize, Debug, Copy, PartialEq, Clone, TryFromByte)]
pub enum PeerType {
    Outbound,
    Inbound,
}


/// A Peer. i.e. another node in the network.
pub struct Peer {
    connection_id: SaitoHash,
    host: Option<[u8; 4]>,
    port: Option<u16>,
    publickey: Option<SaitoPublicKey>,
    request_count: u32,
    is_connected: bool,
    is_connecting: bool,
    peer_type: PeerType,
    // inbound peer
    pub sender: mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>,
    // outbound peer
    pub write_sink:
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::protocol::Message>,
}


impl Peer {
    pub fn new(
        connection_id: SaitoHash,
        host: Option<[u8; 4]>,
        port: Option<u16>,
    ) -> Peer {
        Peer {
            connection_id,
            host,
            port,
            publickey: None,
	    peer_type: PeerType::Outbound;
            request_count: 0,
	    is_connected: false,
	    is_connecting: false,
	    is_from_peer_list: false,
        }
    }

    pub fn get_is_connected(&self) -> bool {
        self.peer_flags.is_connected
    }

    pub fn get_is_connecting(&self) -> bool {
        self.peer_flags.is_connecting
    }

    pub fn get_is_peer_type(&self, PeerType) -> bool {
        return self.peer_type == pt
    }

}


