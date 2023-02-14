use serde::{Serialize, Deserialize};

use super::MysLSP;

#[derive(Serialize, Deserialize)]
struct ClientInfoInitializeRequest {
    name: String,
    version: String
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct InitializeRequest<'a> {
    jsonrpc: &'a str,
    id: u32,
    pub processId: u32,
    pub clientInfo: ClientInfoInitializeRequest,
    pub workspaceFolders: Vec<String>,
}

pub async fn lsp_handler(client: MysLSP) -> MysLSP {
    // send initialize command

    let tx_send = client.tx_send.borrow_mut().clone().unwrap();

    let init_req = InitializeRequest {
        jsonrpc: "2.0",
        id: 1,
        clientInfo: ClientInfoInitializeRequest { name: "mys-test".into(), version: "1.0".into() },
        processId: client._process.borrow().as_ref().unwrap().id().unwrap(),
        workspaceFolders: vec!["/home/suryateja/Projects/mystudio".into()]
    };

    let req_string = serde_json::to_string(&init_req).unwrap();

    tx_send.send(super::ChannelCommData { data: req_string, send_initialized: false }).await.ok();

    client
}