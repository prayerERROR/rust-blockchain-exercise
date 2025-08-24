use crate::core::block::Block;
use crate::core::transaction::Transaction;
use std::sync::mpsc::{channel, Sender, Receiver};

pub enum NetworkMessage {
    NewBlock(Block),
    NewTransaction(Transaction),
    RequestBlocks(usize),
    ResponseBlocks(Vec<Block>),
}

// Network node
pub struct NetworkNode {
    node_id: String,
    tx: Option<Sender<NetworkMessage>>,
    rx: Option<Receiver<NetworkMessage>>,
}

impl NetworkNode {
    pub fn new(node_id: String) -> Self {
        NetworkNode {
            node_id: node_id,
            tx: None,
            rx: None
        }
    }

    pub fn init_channels(&mut self) -> (Sender<NetworkMessage>, Receiver<NetworkMessage>)
    {
            let (tx1, rx1) = channel();
            let (tx2, rx2) = channel();
            self.tx = Some(tx1);
            self.rx = Some(rx2);
            (tx2, rx1)
    }

    pub fn broadcast_block(&self, block: Block) {
        if let Some(ref tx) = self.tx {
            tx.send(NetworkMessage::NewBlock(block));
        }
    }

    pub fn broadcast_transaction(&self, transaction: Transaction) {
        if let Some(ref tx) = self.tx {
            tx.send(NetworkMessage::NewTransaction(transaction));
        }
    }

    pub fn receive_message(&self) -> Option<NetworkMessage> {
        match self.rx {
            Some(ref rx) => rx.try_recv().ok(),
            _ => None,
        }
    }

    pub fn get_node_id(&self) -> String {
        self.node_id.clone()
    }

}