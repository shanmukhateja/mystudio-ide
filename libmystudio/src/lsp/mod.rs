mod handler;

use std::{cell::RefCell, process::Stdio};

use json::object;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::mpsc::{channel, Sender},
    task::{spawn, JoinHandle},
};

use crate::workspace::Workspace;

// thread_local! {static LSP_LIST: RefCell<Vec<MysLSP>> = RefCell::new(vec![]) }

#[derive(Debug)]
pub struct MysLSP {
    pub _process: RefCell<Option<Child>>,
    pub tx_send: RefCell<Option<Sender<ChannelCommData>>>,
    pub x: JoinHandle<()>,
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
pub async fn init_lsp() -> Option<MysLSP> {
    println!("async LSP main");

    let workspace_path = Workspace::get_path();

    if workspace_path.is_empty() {
        return None;
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
    // tokio::spawn(async move {
    //     let x = _spawn().await;

    //     let x = handler::lsp_handler(x).await;

    //     x
    // })
    let x = _spawn().await;

    let x = handler::lsp_handler(x).await;

    Some(x)
}

async fn _spawn() -> MysLSP {
    println!("Spawning LSP instance..");
    let mut process = Command::new("/home/suryateja/.config/mystudio-ide/lsp/rust-analyzer")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(false)
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
        x: y,
    }
}

async fn _setup_listeners(
    stdin: ChildStdin,
    stdout: ChildStdout,
) -> (Sender<ChannelCommData>, JoinHandle<()>) {
    let (tx_send, mut rx_send) = channel::<ChannelCommData>(100);

    let tx_send_clone = tx_send.clone();

    let sender_thread = spawn(async move {
        let mut writer = BufWriter::new(stdin);

        let mut i=0;

        loop {
            let data = rx_send.recv().await;
            if data.is_none() {
                println!("skipping rx_send loop..");
                return;
            }

            println!("\n\ngot data from internal sender pipe: {data:?}");

            let method_name = if i>0 {
                "initialized"
            } else {
                "initialize"
            };

            let obj = object! {
                jsonrpc: "2.0",
                id: 1,
                method: method_name,
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

            let payload = format!(
                "Content-Length: {}\r\n\r\n{}\r\n\r\n",
                obj.to_string().len(),
                obj
            );
            println!("\n\nsending payload to LSP: '{payload}'\n");
            writer.write_all(payload.as_bytes()).await.unwrap();
            writer.flush().await.unwrap();
            i += 1;
        }
    });

    let recv_thread = spawn(async move {
        let mut reader = BufReader::new(stdout);

        loop {
            let mut buf: Vec<u8> = Vec::new();
            // Wait until a message is available instead of constantly polling for a message.
            match reader.read_to_end(&mut buf).await {
                Ok(size) => {
                    if size == 0 {
                        return;
                    }
                    println!("rx_recv got: {:?}", size.to_string());

                    /*let mut buf = Vec::new();
                    let _ = reader.read_to_end(&mut buf);*/

                    if !buf.is_empty() {
                        let str = String::from_utf8(buf);
                        println!("-> {str:?}");

                        // send initialized notification
                        println!("sending initialized notification");
                        tx_send_clone.send(ChannelCommData { data: String::new() }).await.unwrap();
                    } else {
                        println!("buf is empty");
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
    /*tokio::select! {
        _ = sender_thread => {
            println!("foo!");
        },
        _ = recv_thread => {
            println!("bar!");
        }
    };*/

    let _x = tokio::spawn(async {
        /*tokio::select! {
            _ = sender_thread => {
                println!("foo!");
            },
            _ = recv_thread => {
                println!("bar!");
            }
        };*/
        let _ = tokio::join!(sender_thread, recv_thread);
    });
    // tokio::spawn(sender_thread);
    // tokio::spawn(recv_thread);

    println!("finishing up LSP comm listener");
    (tx_send, _x)
}
