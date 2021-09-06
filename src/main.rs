mod app;
mod list;

use crate::app::*;
use rand::Rng;
use relm4::{RelmApp, Sender};
use tokio::runtime::Builder;
use tokio::sync::mpsc::channel;

#[derive(Debug)]
pub enum AsyncHandlerMsg {
    GetItems,
}

impl AsyncHandlerMsg {
    async fn recv(&self, sender: Sender<AppMsg>) {
        use AsyncHandlerMsg::*;

        match self {
            GetItems => {
                for index in 1..10 {
                    let item_sender = sender.clone();
                    let mut rng = rand::thread_rng();
                    let seconds = rng.gen_range(0.0..5.0);

                    tokio::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_secs_f32(seconds)).await;

                        item_sender.send(AppMsg::AddItem(index)).unwrap();
                    });
                }
            }
        }
    }
}

fn main() {
    let (tx, mut rx) = channel::<(AsyncHandlerMsg, Sender<AppMsg>)>(10);

    let rt = Builder::new_multi_thread()
        .worker_threads(8)
        .enable_time()
        .build()
        .unwrap();

    rt.spawn(async move {
        while let Some((msg, sender)) = rx.recv().await {
            msg.recv(sender).await;
        }
    });

    let model = AppModel { async_handler: tx };
    let app = RelmApp::new(model);
    app.run();
}
