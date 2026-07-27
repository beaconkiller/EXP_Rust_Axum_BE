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
    sync::{Arc, atomic::AtomicI32, mpsc::Receiver},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, UnboundedSender},
};
use tokio_tungstenite::{MaybeTlsStream, tungstenite::Message};
use tokio_tungstenite::{WebSocketStream, connect_async};
use uuid::Uuid;

use crate::{global::Global::GL_WS, services::service_sysinfo::SrvSysinfo};

#[derive(Debug)]
pub struct WsClient {
    tx: UnboundedSender<Message>,
}

#[derive(Debug)]
pub struct SrvWs {
    pub status: AtomicI32,
    pub tx: tokio::sync::Mutex<Option<UnboundedSender<Message>>>,
    pub connect_address: String,
    pub sender:
        tokio::sync::Mutex<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
}

impl SrvWs {
    pub async fn init_socket(self: Arc<Self>, socket: WebSocketStream<MaybeTlsStream<TcpStream>>) {
        let self_sys_loop = Arc::clone(&self);

        let (mut sender, mut receiver) = socket.split();
        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

        // let mut guard_tx = self.tx.lock().await;
        // *guard_tx = Some(tx);

        self_sys_loop.start_loop(tx).await;

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if sender.send(msg).await.is_err() {
                    break;
                }
            }
        });

        while let Some(msg) = receiver.next().await {
            // println!("{:?}", msg);
        }
    }

    pub async fn connect_ws(self: Arc<Self>) {
        let self_loop = &self.clone();
        loop {
            match connect_async(self_loop.connect_address.clone()).await {
                Ok((socket, response)) => {
                    self_loop.clone().init_socket(socket).await;
                }
                Err(err) => {
                    println!("connect failed... {:?}", err);
                }
            };

            println!("{:?}", "retrying in 3 seconds...");
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    }

    pub async fn start_loop(self: Arc<Self>, tx: UnboundedSender<Message>) {
        tokio::time::sleep(Duration::from_secs(2)).await;

        tokio::spawn(async move {
            loop {
                let data = SrvSysinfo::get_all_info().await;
                println!("--------------------");
                println!("{:?}", data);
                let data_str = serde_json::to_string(&data).unwrap();
                let msg: Message = Message::Text(data_str.clone().into());
                println!("+++++++++++++++++++");
                println!("{:?}", msg);

                let response = tx.send(msg);
                match response {
                    Ok(()) => {}
                    Err(err) => {
                        println!(" Disconnected... {:?}", err);
                        break;
                    }
                }
                tokio::time::sleep(Duration::from_secs(1)).await
            }
        });

        // tokio::spawn(async move {
        //     loop {
        //         let data = SrvSysinfo::get_all_info().await;
        //         println!("--------------------");
        //         println!("{:?}", data);
        //         let data_str = serde_json::to_string(&data).unwrap();
        //         let msg: Message = Message::Text(data_str.clone().into());
        //         println!("+++++++++++++++++++");
        //         println!("{:?}", msg);

        //         let guard = self.tx.lock().await;
        //         if let Some(tx) = guard.as_ref() {
        //             let _ = tx.send(msg);
        //         }

        //         tokio::time::sleep(Duration::from_secs(1)).await
        //     }
        // });
    }
}
