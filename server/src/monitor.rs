use futures::future;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin;
use std::sync::LazyLock;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::protocol::Connection;
use crate::random::random_uuid;

pub const GAMES: LazyLock<HashMap<Uuid, Game>> = LazyLock::new(|| HashMap::new());

pub struct MpscChannel {
    pub sender: mpsc::Sender<Vec<u8>>,
    pub receiver: mpsc::Receiver<Vec<u8>>,
}

impl MpscChannel {
    pub fn new(buffer_size: usize) -> Self {
        let (sender, receiver) = mpsc::channel(buffer_size);
        MpscChannel { sender, receiver }
    }
}

pub struct ManagerChannel(pub MpscChannel);

pub struct GameChannel(pub MpscChannel);

pub struct Manager {
    pub channel: ManagerChannel,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            channel: ManagerChannel(MpscChannel::new(128)),
        }
    }
}

pub struct Game {
    pub id: Uuid,
    pub channel: GameChannel,
}

impl Game {
    pub fn new() -> Self {
        Game {
            id: random_uuid(),
            channel: GameChannel(MpscChannel::new(128)),
        }
    }
}

pub struct User {
    pub id: Uuid,
    pub game_senders: Vec<mpsc::Sender<u8>>,
    pub connection: Connection,
}

pub async fn monitor_listener(
    mut cancellation_receiver: sync::broadcast::Receiver<()>,
    listener: TcpListener,
) {
    let cancellation_receiver_forward = cancellation_receiver.resubscribe();
    let task_f = async move {
        loop {
            let accept_r = listener.accept().await;

            match accept_r {
                Ok(val) => {
                    tokio::spawn(monitor_client(
                        cancellation_receiver_forward.resubscribe(),
                        val.0,
                        val.1,
                    ));
                }
                Err(err) => {
                    log::error!("Failed to accept TCP connection; {:#}", err);
                    break;
                }
            }
        }
    };
    let task_f = pin::pin!(task_f);

    let cancellation_f = cancellation_receiver.recv();
    let cancellation_f = pin::pin!(cancellation_f);

    future::select(cancellation_f, task_f).await;

    log::debug!("monitor_listener terminated");
    // todo: cleanup here
}

pub async fn monitor_manager(mut cancellation_receiver: sync::broadcast::Receiver<()>) {
    let task_f = async {
        let _manager = Manager::new();
        // todo: loop await on manager channel receiver
    };
    let task_f = pin::pin!(task_f);

    let cancellation_f = cancellation_receiver.recv();
    let cancellation_f = pin::pin!(cancellation_f);

    future::select(cancellation_f, task_f).await;

    log::debug!("monitor_manager terminated");
    // todo: cleanup here
}

async fn monitor_client(
    mut cancellation_receiver: sync::broadcast::Receiver<()>,
    tcp_stream: TcpStream,
    socket_addr: SocketAddr,
) {
    let task_f = async {
        Connection::new(tcp_stream, socket_addr);
        // todo: loop await on tcp input data
    };
    let task_f = pin::pin!(task_f);

    let cancellation_f = cancellation_receiver.recv();
    let cancellation_f = pin::pin!(cancellation_f);

    future::select(cancellation_f, task_f).await;

    log::debug!("monitor_client terminated");
    // todo: cleanup here
}
