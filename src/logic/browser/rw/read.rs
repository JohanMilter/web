use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    #[serde(rename = "type")]
    pub r#type: String,
}
