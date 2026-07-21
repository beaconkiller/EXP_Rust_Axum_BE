use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    http::{header::DATE, response},
    response::Response,
};
use futures_util::{
    SinkExt, StreamExt,
    lock::Mutex,
    stream::{SplitSink, SplitStream},
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::{
    collections::HashSet,
    sync::{Arc, atomic::AtomicI32},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use crate::{global::Global::GL_WS, services::service_sysinfo::SrvSysinfo};

#[derive(Debug)]
pub struct ModelConn {
    pub sender: Arc<tokio::sync::Mutex<SplitSink<WebSocket, Message>>>,
    pub id: String,
}

#[derive(Debug)]
pub struct ModelClient {
    pub tx: mpsc::UnboundedSender<Message>,
    pub id: String,
}

#[derive(Debug)]
pub struct WsClient {
    tx: UnboundedSender<Message>,
}

#[derive(Debug)]
pub struct SrvWs {
    pub status: AtomicI32,
    pub arr_clients: tokio::sync::Mutex<Vec<ModelClient>>,
    pub tx: tokio::sync::Mutex<Option<UnboundedSender<Message>>>,
}

impl SrvWs {
    pub async fn init(self: Arc<Self>) {
        let (socket, response) = connect_async("ws://127.0.0.1:2121/ws").await.unwrap();
        let (mut sender, receiver) = socket.split();
        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

        {
            let mut tmp_tx = self.tx.lock().await;
            *tmp_tx = Some(tx);
        }

        tokio::spawn(async move {
            let mut sender = sender;
            while let Some(msg) = rx.recv().await {
                if sender.send(msg).await.is_err() {
                    println!("Disconnected");
                    break;
                }
            }
        });

        self.start_loop();

        // tokio::spawn(async move {
        //     let mut receiver = receiver;
        //     while let Some(msg) = receiver.next().await {
        //         match msg {
        //             Ok(Message::Text(text)) => {
        //                 println!("Received: {}", text);
        //             }
        //             Ok(_) => {}
        //             Err(e) => {
        //                 println!("Receive error: {}", e);
        //                 break;
        //             }
        //         }
        //     }
        // });
    }

    pub fn start_loop(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                let data = SrvSysinfo::get_all_info().await;
                println!("{:?}", data);
                let data_str = serde_json::to_string(&data).unwrap();
                let msg: Message = Message::Text(data_str.clone().into());

                let guard = self.tx.lock().await;
                if let Some(tx) = guard.as_ref() {
                    let _ = tx.send(msg);
                }

                tokio::time::sleep(Duration::from_secs(1)).await
            }
        });
    }

    // pub async fn handler(ws: WebSocketUpgrade) -> Response {
    //     ws.on_upgrade(|socket| async move {
    //         GL_WS.fn_socket(socket).await;
    //     })
    // }

    // async fn fn_socket(&self, socket: WebSocket) {
    //     let (sender, receiver) = socket.split();

    //     let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    //     self.add_client(tx).await;

    //     tokio::spawn(async move {
    //         let mut sender = sender;
    //         while let Some(msg) = rx.recv().await {
    //             if sender.send(msg).await.is_err() {
    //                 break;
    //             }
    //             println!("{:?}", "new client connected");
    //         }
    //     });
    // }

    pub async fn add_client(&self, tx: UnboundedSender<Message>) {
        let id = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis())
        .to_string();

        self.arr_clients.lock().await.push({
            ModelClient {
                tx: tx,
                id: id.into(),
            }
        });
        println!("{:?}", self.arr_clients);
    }

    async fn remove_clients(&self, hash_remove: HashSet<String>) {
        let mut clients = self.arr_clients.lock().await;
        clients.retain(|client| !hash_remove.contains(&client.id));

        let mut str_removed: String = "".to_string();
        for el in hash_remove {
            str_removed += &(", ".to_string() + &el.to_string()).to_string();
        }
        println!("{:?}", str_removed + " removed");
    }
}
