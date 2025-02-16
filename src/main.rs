use clap::Parser;
use std::time::{Duration, SystemTime};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    concurrent: u32,

    #[arg(short, long, default_value_t = 1)]
    recursion: u32,

    #[arg(short, long, default_value_t = 1)]
    delay: u64,

    #[arg(short, long)]
    url: String,
}

use std::sync::Arc;
#[tokio::main]
async fn main() {
    let args = Arc::new(Args::parse());

    // web_test -u https://a -c 10 -r 4 -1

    let mut vec = Vec::new();

    for x in 0..args.recursion {
        vec.append(&mut spawn2(args.clone()).await);
        if x + 1 != args.recursion {
            tokio::time::sleep(std::time::Duration::from_secs(args.delay)).await;
        }
    }

    let times = vec
        .into_iter()
        .filter_map(|a| a.map(|a| a.0.as_millis()).ok())
        .collect::<Vec<_>>();

    let mut csv = times
        .into_iter()
        .map(|a| format!("{a},"))
        .collect::<String>();

    csv.pop();

    std::fs::write("times.csv", csv).expect("cant write file");
}
use reqwest::Response;

use futures::stream;
use futures::StreamExt;
async fn spawn2(args: Arc<Args>) -> Vec<Result<(Duration, Response), anyhow::Error>> {
    stream::iter(0..args.concurrent)
        .map(|_| {
            tokio::spawn({
                let args = args.clone();
                async move { inner(args).await }
            })
        })
        .buffer_unordered(args.concurrent as usize)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|x| {
            let x = x?;
            let x = Ok((x.0, x.1?));
            x
        })
        .collect::<Vec<Result<(Duration, Response), anyhow::Error>>>()
}
type Data = (Duration, Result<Response, reqwest::Error>);

async fn inner(args: Arc<Args>) -> Data {
    let then = SystemTime::now();
    let request = reqwest::get(args.url.clone()).await;
    let elapsed = then.elapsed().unwrap();

    (elapsed, request)
}
