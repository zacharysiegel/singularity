use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::LazyLock;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use uuid::Uuid;

pub const GAMES: LazyLock<HashMap<Uuid, Game>> = LazyLock::new(|| HashMap::new());

pub struct MpscChannel {
    pub sender: mpsc::Sender<Vec<u8>>,
    pub receiver: mpsc::Receiver<Vec<u8>>,
}
pub struct ManagerChannel(pub MpscChannel);
pub struct GameChannel(pub MpscChannel);

pub struct Manager {
    pub channel: ManagerChannel,
}

pub struct Game {
    pub id: Uuid,
    pub channel: GameChannel,
}

pub struct User {
    pub id: Uuid,
    pub game_senders: Vec<mpsc::Sender<u8>>,
    pub connection: Connection,
}

pub struct Connection {
    pub tcp_stream: TcpStream,
    pub socket_addr: SocketAddr,
}

pub async fn monitor_listener(listener: TcpListener) {
    loop {
        let accept_r = listener.accept().await;

        match accept_r {
            Ok(val) => {
                tokio::spawn(monitor_client(val.0, val.1));
            }
            Err(err) => {
                log::error!("Failed to accept TCP connection; {:#}", err);
                break;
            }
        }
    }
}

async fn monitor_client(tcp_stream: TcpStream, socket_addr: SocketAddr) {
    Connection {
        tcp_stream,
        socket_addr,
    };
}
