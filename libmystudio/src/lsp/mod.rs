mod handler;

use std::{cell::RefCell, process::Stdio};

use json::object;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::mpsc::{channel, Sender},
    task::JoinHandle,
};

use crate::workspace::Workspace;

// thread_local! {static LSP_LIST: RefCell<Vec<MysLSP>> = RefCell::new(vec![]) }

#[derive(Debug)]
pub struct MysLSP {
    pub _process: RefCell<Option<Child>>,
    pub tx_send: RefCell<Option<Sender<ChannelCommData>>>,
    pub handle: Option<JoinHandle<()>>,
}

#[derive(Debug)]
pub struct ChannelCommData {
    pub data: String,
    pub send_initialized: bool,
}

impl ToString for ChannelCommData {
    fn to_string(&self) -> String {
        self.data.clone()
    }
}

// #[tokio::main]
pub async fn init_lsp() -> Child {
    println!("async LSP main");

    let workspace_path = Workspace::get_path();

    if workspace_path.is_empty() {
        // println!("workspace not set, returing");
        Workspace::update_path("/home/suryateja/Projects/mystudio".into());
        // return None;
    }

    // LSP_LIST.with(|e| {
    //     (*e.borrow_mut()).as_mut().unwrap().push(MysLSP {
    //         _process: RefCell::new(None),
    //         tx_recv: RefCell::new(None),
    //         tx_send: RefCell::new(None),
    //     });
    // });

    // tokio::join! {
    //     tokio::spawn(async move {
    //         let x = _spawn().await;

    //         let x = handler::lsp_handler(x).await;

    //         x
    //     })
    // }
    // .0
    // .unwrap(),
    tokio::spawn(async move {
        let x = _spawn().await;

        let x = handler::lsp_handler(x).await;

        // x
        x._process.take().unwrap()
    })
    .await
    .unwrap()
    // let x = _spawn().await;

    // let x = handler::lsp_handler(x).await;

    // Some(x)
}

async fn _spawn() -> MysLSP {
    println!("Spawning LSP instance..");
    let mut process = Command::new("/home/suryateja/.config/mystudio-ide/lsp/rust-analyzer-new")
        // .arg("start")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .args([
            "--log-file",
            "/home/suryateja/.config/mystudio-ide/lsp/log.log",
        ])
        .spawn()
        .expect("Unable to spawn LSP server");

    let process_id = process.id().unwrap();
    println!("Process ID: {process_id}");

    let stdin = process.stdin.take().expect("Unable to get stdin channel");
    let stdout = process.stdout.take().expect("Unable to get stdout channel");

    println!("setting LSP comm listners");

    let (x, y) = _setup_listeners(stdin, stdout).await;

    MysLSP {
        _process: RefCell::new(Some(process)),
        tx_send: RefCell::new(Some(x)),
        handle: y,
    }
}

async fn _setup_listeners(
    stdin: ChildStdin,
    stdout: ChildStdout,
) -> (Sender<ChannelCommData>, Option<JoinHandle<()>>) {
    let (tx_send, mut rx_send) = channel::<ChannelCommData>(100);

    let _tx_send_clone = tx_send.clone();

    let sender_thread: JoinHandle<Option<()>> = tokio::spawn(async move {
        let mut writer = BufWriter::new(stdin);

        loop {
            let data = rx_send.recv().await;
            if let Some(recv_data) = data {
                // println!("\n\ngot data from internal sender pipe: \"{recv_data:?}\"");

                let obj = if !recv_data.send_initialized {
                    object! {
                        jsonrpc: "2.0",
                        id: 1,
                        method: "initialize",
                        params: {
                            clientInfo: {
                                name: "mystudio-ide",
                                version: "1.0"
                            },
                            capabilities: {},
                            rootPath: "/home/suryateja/Projects/mystudio",
                            locale: "en"
                        }
                    }
                } else {
                    println!("sending INITIALIZED");
                    object! {method: "initialized", params: {} }
                };
                // tokio::time::sleep(Duration::from_secs(3)).await;

                // let payload = "Content-Length: 2\r\n\r\n{}".to_string();
                // println!("\n\nsending payload to LSP");
                let payload = format!("Content-Length: {}\r\n\r\n{}", obj.to_string().len(), obj);
                println!("\n\nsending payload to LSP: '{payload}'\n");
                writer.write_all(payload.as_bytes()).await.unwrap();
                writer.flush().await.unwrap();

                let init_obj = object! {method: "initialized", params: {} };
                let payload = format!(
                    "Content-Length: {}\r\n\r\n{}",
                    init_obj.to_string().len(),
                    init_obj
                );
                println!("\n\nsending payload to LSP: '{payload}'\n");
                writer.write_all(payload.as_bytes()).await.unwrap();
                writer.flush().await.unwrap();
            }
        }
    });

    let recv_thread = tokio::spawn(async move {
        let mut reader = BufReader::new(stdout);

        loop {
            let mut buf: Vec<u8> = Vec::new();
            // Wait until a message is available instead of constantly polling for a message.
            match reader.read_to_end(&mut buf).await {
                Ok(size) => {
                    if size > 0 {
                        println!("rx_recv got: {:?}", size.to_string());

                        /*let mut buf = Vec::new();
                        let _ = reader.read_to_end(&mut buf);*/

                        if !buf.is_empty() {
                            let str = String::from_utf8(buf);
                            println!("-> {str:?}");

                            // send initialized notification
                            // println!("sending initialized notification");
                            // _tx_send_clone
                            //     .send(ChannelCommData {
                            //         data: String::new(),
                            //         send_initialized: true,
                            //     })
                            //     .await
                            //     .unwrap();
                        } else {
                            println!("buf is empty");
                        }
                    }
                }
                Err(err) => {
                    eprintln!("rx_recv: quitting loop {err}");
                    break;
                }
            }
        }
    });

    println!("setting up LSP comm select! macro");

    let _x = tokio::spawn(async {
        println!("umm creating join for send,recv");
        tokio::select! {
            _ = sender_thread => {
                println!("sender_thread finished :(");
            },
            _ = recv_thread => {
                println!("recv_thread finished :(");
            }
        };
        // let _ = tokio::join!(sender_thread, recv_thread);
        println!("umm what to do now??");
    });
    // tokio::spawn(sender_thread);
    // tokio::spawn(recv_thread);

    println!("finishing up LSP comm listener");
    (tx_send, Some(_x))
}
