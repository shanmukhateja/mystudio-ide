mod handler;

use std::{cell::RefCell, process::Stdio};

use json::object;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::mpsc::{channel, Sender},
    task::spawn,
};

use crate::workspace::Workspace;

thread_local! {static LSP_LIST: RefCell<Option<Vec<MysLSP>>> = RefCell::new(None) }

pub struct MysLSP {
    pub _process: RefCell<Option<Child>>,
    pub tx_send: RefCell<Option<Sender<ChannelCommData>>>,
    pub tx_recv: RefCell<Option<Sender<ChannelCommData>>>,
}

#[derive(Debug)]
pub struct ChannelCommData {
    pub data: String,
}

impl ToString for ChannelCommData {
    fn to_string(&self) -> String {
        self.data.clone()
    }
}

// #[tokio::main]
pub async fn init_lsp() {
    println!("async LSP main");

    let workspace_path = Workspace::get_path();

    if workspace_path.is_empty() {
        return;
    }

    // LSP_LIST.with(|e| {
    //     (*e.borrow_mut()).as_mut().unwrap().push(MysLSP {
    //         _process: RefCell::new(None),
    //         tx_recv: RefCell::new(None),
    //         tx_send: RefCell::new(None),
    //     });
    // });

    println!("Spawning LSP instance..");
    let x = _spawn().await;

    handler::lsp_handler(x).await;
}

async fn _spawn() -> MysLSP {
    let mut process = Command::new("/home/suryateja/.config/mystudio-ide/lsp/rust-analyzer")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Unable to spawn LSP server");

    let stdin = process.stdin.take().expect("Unable to get stdin channel");
    let stdout = process.stdout.take().expect("Unable to get stdout channel");

    println!("setting LSP comm listners");
    let (x, y) = _setup_listeners(stdin, stdout).await;

    MysLSP {
        _process: RefCell::new(Some(process)),
        tx_send: RefCell::new(Some(x)),
        tx_recv: RefCell::new(Some(y)),
    }
}

async fn _setup_listeners(
    stdin: ChildStdin,
    stdout: ChildStdout,
) -> (Sender<ChannelCommData>, Sender<ChannelCommData>) {
    let (tx_send, mut rx_send) = channel::<ChannelCommData>(100);
    let (tx_recv, mut rx_recv) = channel::<ChannelCommData>(100);

    let sender_thread = spawn(async move {
        let mut writer = BufWriter::new(stdin);

        loop {
            let data = rx_send.recv().await;
            if data.is_none() {
                println!("breaking rx_send loop..");
                break;
            }

            println!("got data from internal sender pipe: {data:?}");

            let obj = object! {
                jsonrpc: "2.0",
                id: 1,
                method: "initialize",
                params: {
                    jsonrpc: "2.0",
                    clientInfo: {
                        name: "mystudio-ide",
                        version: "1.0"
                    },
                    capabilities: {},
                    rootPath: "/home/suryateja/Projects/mdt",
                    locale: "en"
                }
            };

            let payload = format!("Content-Length: {}\r\n\r\n{}\r\n\r\n", obj.to_string().len(), obj);
            writer.write_all(payload.as_bytes()).await.ok();
        }
    });

    let recv_thread = spawn(async move {
        let mut reader = BufReader::new(stdout);

        loop {
            let data = rx_recv.recv().await;
            if data.is_none() {
                println!("breaking rx_recv loop..");
                break;
            }

            let mut buf = vec![];
            reader.read_to_end(&mut buf).await.ok();
        }
    });

    println!("setting up LSP comm select! macro");
    /*tokio::select! {
        _ = sender_thread => {
            println!("foo!");
        },
        _ = recv_thread => {
            println!("bar!");
        }
    };*/

    tokio::join!(sender_thread, recv_thread);

    println!("finishing up LSP comm listener");

    (tx_send, tx_recv)
}
