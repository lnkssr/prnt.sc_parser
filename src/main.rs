use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::Rng;
use rand::SeedableRng;
use reqwest::{header::HeaderValue, Client};
use scraper::{Html, Selector};
use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio;
use tokio::sync::Semaphore;

const MAX_CONCURRENT_REQUESTS: usize = 10; // Number of concurrent requests
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:90.0) Gecko/20100101 Firefox/90.0",
    "Mozilla/5.0 (Linux; Android 10; SM-G973U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.120 Mobile Safari/537.36 SamsungBrowser/14.0"
];

async fn fetch_and_parse_image(
    client: Arc<Client>,
    url: String,
    output_dir: Arc<Mutex<String>>,
    semaphore: Arc<Semaphore>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _permit = semaphore.acquire().await?;
    let user_agent = USER_AGENTS
        .iter()
        .choose(&mut rand::thread_rng())
        .unwrap_or(&USER_AGENTS[0]);
    let user_agent_value = HeaderValue::from_static(user_agent);
    let resp = client
        .get(&url)
        .header("User-Agent", user_agent_value)
        .send()
        .await?
        .text()
        .await?;

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
            println!("[+] Image found for token {}. Download...", url);

            let img = client.get(&image_url).send().await?.bytes().await?;
            let file_name = url.split('/').last().unwrap_or("image");
            let file_path = format!(
                "{}/{}{}",
                output_dir.lock().unwrap(),
                file_name,
                &image_url[image_url.rfind('.').unwrap_or(0)..]
            );
            let mut file = File::create(file_path)?;
            file.write_all(&img)?;
        } else {
            println!("[+] Nothing found by token {}", url);
        }
    } else {
        println!("[+] Cloudflare block");
    }

    Ok(())
}

async fn parse(
    client: Arc<Client>,
    output_dir: Arc<Mutex<String>>,
    semaphore: Arc<Semaphore>,
) -> Result<(), Box<dyn std::error::Error>> {
    let symbols = "abcdefghiklmnopqrstvxyz123456789";
    let mut rng = StdRng::from_entropy(); // Using StdRng which is Send

    loop {
        let token: String = (0..rng.gen_range(3..=8))
            .map(|_| symbols.chars().choose(&mut rng).unwrap())
            .collect();

        let url = format!("https://prnt.sc/{}", token);

        let client = client.clone();
        let output_dir = output_dir.clone();
        let semaphore = semaphore.clone();

        tokio::spawn(async move {
            if let Err(e) = fetch_and_parse_image(client, url, output_dir, semaphore).await {
                eprintln!("Error: {}", e);
            }
        });

        // Sleep to prevent hammering the server
        tokio::time::sleep(Duration::from_secs(2)).await; // Increased delay
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let proc_count = args
        .get(1)
        .map(|s| s.parse::<usize>())
        .unwrap_or_else(|| Ok(1))?;

    let output_dir = Arc::new(Mutex::new("output".to_string()));
    std::fs::create_dir_all("output")?;

    let client = Arc::new(
        Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0")
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
