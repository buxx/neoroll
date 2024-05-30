use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
};

use crossbeam::channel::{unbounded, Receiver, RecvError, SendError, Sender};
use uuid::Uuid;

use crate::server::{ClientMessage, ServerMessage, ServerMessageEnveloppe};

pub struct Gateways {
    server_sender: Sender<ServerMessageEnveloppe>,
    server_receiver: Receiver<ServerMessageEnveloppe>,
    clients_sender: Sender<ClientMessageEnveloppe>,
    clients_receiver: Receiver<ClientMessageEnveloppe>,
    clients: Arc<RwLock<HashMap<ClientId, Sender<ServerMessage>>>>,
}

impl Gateways {
    pub fn new() -> Self {
        let (server_sender, server_receiver): (
            Sender<ServerMessageEnveloppe>,
            Receiver<ServerMessageEnveloppe>,
        ) = unbounded();
        let (clients_sender, clients_receiver): (
            Sender<ClientMessageEnveloppe>,
            Receiver<ClientMessageEnveloppe>,
        ) = unbounded();
        Self {
            server_sender,
            server_receiver,
            clients_sender,
            clients_receiver,
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Send to client(s)
    pub fn send(
        &self,
        message: ServerMessageEnveloppe,
    ) -> Result<(), SendError<ServerMessageEnveloppe>> {
        self.server_sender.send(message)
    }

    /// Receive from clients
    pub fn receive(&self) -> Result<ClientMessageEnveloppe, RecvError> {
        self.clients_receiver.recv()
    }

    /// Register new client and return `Gateway` permitting to client to send
    /// and receive message from server
    pub fn register(&mut self) -> Gateway {
        let (server_sender, server_receiver): (Sender<ServerMessage>, Receiver<ServerMessage>) =
            unbounded();
        let (client_sender, client_receiver): (Sender<ClientMessage>, Receiver<ClientMessage>) =
            unbounded();
        let client_id = ClientId::new();

        let clients_sender = self.clients_sender.clone();
        thread::spawn(move || {
            while let Ok(message) = client_receiver.recv() {
                // TODO unwrap
                clients_sender
                    .send(ClientMessageEnveloppe(client_id, message))
                    .unwrap()
            }
        });

        self.clients
            .write()
            // TODO: unwrap
            .unwrap()
            .insert(client_id, server_sender);

        Gateway::new(client_id, server_receiver, client_sender)
    }

    pub fn start(&self) {
        let server_receiver = self.server_receiver.clone();
        let clients = Arc::clone(&self.clients);
        thread::spawn(move || {
            while let Ok(enveloppe) = server_receiver.recv() {
                match enveloppe {
                    ServerMessageEnveloppe::Broadcast(message) => {
                        // TODO: unwrap
                        for client_sender in clients.read().unwrap().values() {
                            client_sender.send(message.clone()).unwrap()
                        }
                    }
                    ServerMessageEnveloppe::To(client_id, message) => {
                        // TODO: unwrap
                        if let Some(client_sender) = clients.read().unwrap().get(&client_id) {
                            client_sender.send(message).unwrap()
                        }
                    }
                }
            }
        });
    }
}

impl Default for Gateways {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Gateway {
    client_id: ClientId,
    server_receiver: Receiver<ServerMessage>,
    client_sender: Sender<ClientMessage>,
}

impl Gateway {
    pub fn new(
        client_id: ClientId,
        server_receiver: Receiver<ServerMessage>,
        client_sender: Sender<ClientMessage>,
    ) -> Self {
        Self {
            client_id,
            server_receiver,
            client_sender,
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    /// Send to server
    pub fn send(&self, message: ClientMessage) -> Result<(), SendError<ClientMessage>> {
        self.client_sender.send(message)
    }

    /// Receive (blocking) from server
    pub fn receive(&self) -> Result<ServerMessage, RecvError> {
        self.server_receiver.recv()
    }

    /// Take messages iif any from server
    pub fn read(&self) -> Vec<ServerMessage> {
        self.server_receiver.try_iter().collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(Uuid);

impl ClientId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ClientId {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ClientMessageEnveloppe(pub ClientId, pub ClientMessage);
