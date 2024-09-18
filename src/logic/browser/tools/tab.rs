use std::net::Ipv4Addr;

use futures::{stream::SplitSink, SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};

use crate::{logic::browser::{drivers::behaviors::{DriverReadBehavior, DriverWriteBehavior}, rw::read::{ClickElementResponse, DOMAttributeModifiedResponse, GetDocumentResponse, NavigateResponse, ResolveNodeResponse}, By}, utils::types::{error::Error, result::Result}};

type WSStream = (
    SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
    futures::stream::SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>,
);
pub struct Tab<D: DriverReadBehavior + DriverWriteBehavior> {
    driver: std::marker::PhantomData<D>,
    stream: WSStream,
}
impl<D: DriverReadBehavior + DriverWriteBehavior> Tab<D> {
    pub async fn send_command<T>(&mut self, command: serde_json::Value) -> Result<T>
    where
        T: DeserializeOwned, {
        println!("Sending command: \n{}", command);
        self.stream
            .0
            .send(Message::Text(command.to_string()))
            .await?;

        println!("Waiting for repsonse:");
        // Wait for the response
        while let Some(msg) = self.stream.1.next().await {
            match msg {
                Ok(Message::Text(text)) => match serde_json::from_str::<T>(&text) {
                    Ok(response) => {
                        println!("Deserializing:\n{text}\n");
                        return Ok(response);
                    }
                    Err(_) => eprintln!("Can't deserialize:\n{text}\n"),
                },
                Ok(_) => continue,
                Err(_) => return Err(Error::BadStream),
            }
        }
        Err(Error::BadStream)
    }
    pub async fn init(address: Ipv4Addr, port: u16) -> Result<Tab<D>> {
        let targets = D::get_devtools_targets(address, port).await?;
        println!("{}", targets.len());
        let target = &targets[targets.len() - 2];
        let ws_url = &target.web_socket_debugger_url;

        Ok(Self {
            driver: std::marker::PhantomData::<D>,
            stream: connect_async
            (ws_url)
                .await
                .expect("Failed to connect")
                .0
                .split(),
        })
    }
}

impl<D: DriverReadBehavior + DriverWriteBehavior> Tab<D>  {
    pub async fn navigate(&mut self, url: &str) -> Result<NavigateResponse> {
        self.send_command::<NavigateResponse>(D::navigate(url))
            .await
    }
    pub async fn get_element(&mut self, by: By) -> Result<DOMAttributeModifiedResponse> {
        let document = self.get_document().await?;
        let node_id = document.result.root.node_id;

        self.send_command::<DOMAttributeModifiedResponse>(D::get_element(by, node_id))
            .await
    }
    pub async fn get_document(&mut self) -> Result<GetDocumentResponse> {
        self.send_command::<GetDocumentResponse>(D::get_document())
            .await
    }
    pub async fn resolve_node(&mut self, id: u32) -> Result<ResolveNodeResponse> {
        self.send_command::<ResolveNodeResponse>(D::resolve_node(id))
            .await
    }
    pub async fn click_element(&mut self, id: &str) -> Result<ClickElementResponse> {
        self.send_command::<ClickElementResponse>(D::click_element(id))
            .await
    }
}