use rand::prelude::IteratorRandom;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use reqwest::{header::HeaderValue, Client};
use scraper::{Html, Selector};
use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tokio::sync::Semaphore;

const MAX_CONCURRENT_REQUESTS: usize = 10;
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.3",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:39.0) Gecko/20100101 Firefox/39.0",
    "Mozilla/5.0 (Linux; Android 5.0; SAMSUNG-SM-G900A Build/LRX21T) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/2.1 Chrome/34.0.1847.76 Mobile Safari/537.36"
];

async fn fetch_and_parse_image(
    client: Arc<Client>,
    url: String,
    output_dir: &str,
    semaphore: Arc<Semaphore>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _permit = semaphore.acquire().await?;
    let resp = client.get(&url).send().await?.text().await?;

    let image_url = {
        let document = Html::parse_document(&resp);
        let selector = Selector::parse("meta[property='og:image']").unwrap();
        document
            .select(&selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(String::from)
    };

    if let Some(image_url) = image_url {
        if image_url.starts_with("http") {
            println!("[+] Image found for token {}. Downloading...", url);

            let img = client.get(&image_url).send().await?.bytes().await?;
            let file_name = url.split('/').last().unwrap_or("image");
            let file_path = format!(
                "{}/{}{}",
                output_dir,
                file_name,
                &image_url[image_url.rfind('.').unwrap_or(0)..]
            );
            let mut file = File::create(file_path)?;
            file.write_all(&img)?;
        } else {
            println!("[-] No valid image found for token {}", url);
        }
    } else {
        println!("[-] Could not retrieve image for token {}", url);
    }

    Ok(())
}

async fn parse(
    client: Arc<Client>,
    output_dir: Arc<String>,
    semaphore: Arc<Semaphore>,
) -> Result<(), Box<dyn std::error::Error>> {
    let symbols = "abcdefghiklmnopqrstvxyz123456789";
    let mut rng = StdRng::from_entropy();

    loop {
        let token: String = (0..rng.gen_range(3..=8))
            .map(|_| symbols.chars().choose(&mut rng).unwrap())
            .collect();

        let url = format!("https://prnt.sc/{}", token);

        let client = client.clone();
        let output_dir = output_dir.clone();
        let semaphore = semaphore.clone();

        tokio::spawn(async move {
            if let Err(e) = fetch_and_parse_image(client, url, &output_dir, semaphore).await {
                eprintln!("Error: {}", e);
            }
        });

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let proc_count = args
        .get(1)
        .map(|s| s.parse::<usize>())
        .unwrap_or_else(|| Ok(1))?;

    let output_dir = Arc::new("output".to_string());
    std::fs::create_dir_all(&*output_dir)?;

    let user_agent = USER_AGENTS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&USER_AGENTS[0]);
    let client = Arc::new(
        Client::builder()
            .user_agent(HeaderValue::from_static(user_agent))
            .build()?,
    );
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    let mut handles = vec![];

    for _ in 0..proc_count {
        let client = client.clone();
        let output_dir = output_dir.clone();
        let semaphore = semaphore.clone();

        let handle = tokio::spawn(async move {
            if let Err(e) = parse(client, output_dir, semaphore).await {
                eprintln!("Error: {}", e);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}
