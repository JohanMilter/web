use std::{net::Ipv4Addr, path::Path};

use behaviors::DriverBehavior;
use futures::{stream::SplitSink, SinkExt, StreamExt};
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::{net::TcpStream, process::Command};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{http::uri::Port, Message},
    WebSocketStream,
};

use crate::utils::types::{error::Error, result::Result};

pub mod behaviors;
pub mod drivers;

pub struct Browser<D: DriverBehavior> {
    driver: std::marker::PhantomData<D>,
    process: tokio::process::Child,
    ipv4addr: Ipv4Addr,
    port: u16,
}
impl<D: DriverBehavior> Drop for Browser<D> {
    fn drop(&mut self) {
        _ = self.process.start_kill();
    }
}
impl<D: DriverBehavior> Browser<D> {
    pub async fn open(port: u16) -> Result<(Self, Tab<D>)> {
        //The browser should be open on the local machine
        const LOCAL_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

        //Build the instance
        Ok((
            Self {
                driver: std::marker::PhantomData::<D>,
                process: D::open_process(LOCAL_ADDRESS, port),
                ipv4addr: LOCAL_ADDRESS,
                port,
            },
            Tab::init(LOCAL_ADDRESS, port).await.unwrap(),
        ))
    }
    pub async fn new_tab(&self) -> Result<Tab<D>> {
        let url = format!("http://{}:{}/json/new", self.ipv4addr, self.port);
        println!("{:?}", url);
        let resp = reqwest::get(&url).await.unwrap();
        println!("{:?}", resp);
        println!("{:?}", url);
        // Deserialize the response into DevToolsTarget
        Tab::init(self.ipv4addr, self.port).await
    }
}
type WSStream = (
    SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>,
    futures::stream::SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>,
);
pub struct Tab<D: DriverBehavior> {
    driver: std::marker::PhantomData<D>,
    stream: WSStream,
}
impl<D: DriverBehavior> Tab<D> {
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
            stream: connect_async(ws_url)
                .await
                .expect("Failed to connect")
                .0
                .split(),
        })
    }
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DevToolsTarget {
    pub description: String,
    #[serde(rename = "devtoolsFrontendUrl")]
    pub devtools_frontend_url: String,
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub target_type: String,
    pub url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: String,
}
pub enum By {
    Id(String),
    XPath(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigateResponse {
    pub id: u32,
    pub result: NavigateResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigateResult {
    #[serde(rename = "frameId")]
    pub frame_id: String,
    #[serde(rename = "loaderId")]
    pub loader_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DOMAttributeModifiedResponse {
    pub method: String,
    pub params: AttributeModifiedParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeModifiedParams {
    #[serde(rename = "nodeId")]
    pub node_id: u32,
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDocumentResponse {
    pub id: u32,
    pub result: GetDocumentResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDocumentResult {
    pub root: Node,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    #[serde(rename = "nodeId")]
    pub node_id: u32,
    #[serde(rename = "parentId")]
    pub parent_id: Option<u32>,
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: u32,
    #[serde(rename = "nodeType")]
    pub node_type: u32,
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "localName")]
    pub local_name: String,
    #[serde(rename = "nodeValue")]
    pub node_value: String,
    #[serde(rename = "childNodeCount")]
    pub child_node_count: Option<u32>,
    pub children: Option<Vec<Node>>,
    pub attributes: Option<Vec<String>>,
    #[serde(rename = "frameId")]
    pub frame_id: Option<String>,
    #[serde(rename = "documentURL")]
    pub document_url: Option<String>,
    #[serde(rename = "baseURL")]
    pub base_url: Option<String>,
    #[serde(rename = "publicId")]
    pub public_id: Option<String>,
    #[serde(rename = "systemId")]
    pub system_id: Option<String>,
    #[serde(rename = "xmlVersion")]
    pub xml_version: Option<String>,
    #[serde(rename = "compatibilityMode")]
    pub compatibility_mode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveNodeResponse {
    pub id: u32,
    pub result: ResolveNodeResult,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveNodeResult {
    pub object: ResolveNodeResultObject,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveNodeResultObject {
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    subtype: String,
    #[serde(rename = "className")]
    pub class_name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClickElementResponse {
    pub id: u32,
    pub result: ClickElementResult,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClickElementResult {
    pub result: ClickElementResultResult,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClickElementResultResult {
    pub r#type: String,
}
