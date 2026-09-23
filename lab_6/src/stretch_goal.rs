use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;

pub async fn run() {
    let urls = vec![
        "https://example.com",
        "https://www.rust-lang.org",
        "https://doc.rust-lang.org",
        "https://crates.io",
        "https://github.com",
        "https://docs.rs",
        "https://tokio.rs",
        "https://httpbin.org/html",
        "https://www.wikipedia.org",
        "https://www.mozilla.org",
    ];

    let semaphore = Arc::new(Semaphore::new(3));

    let mut handles = Vec::new();

    for url in urls {
        let sem = semaphore.clone();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.unwrap();

            match reqwest::get(url).await {
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    let words = body.split_whitespace().count();

                    (url.to_string(), words)
                }
                Err(_) => (url.to_string(), 0),
            }
        });

        handles.push(handle);
    }

    let mut file = File::create("summary.csv").await.unwrap();

    file.write_all(b"URL,WordCount\n").await.unwrap();

    println!("Summary");

    for handle in handles {
        let (url, words) = handle.await.unwrap();

        println!("{} -> {}", url, words);

        let line = format!("{},{}\n", url, words);

        file.write_all(line.as_bytes()).await.unwrap();
    }

    println!("summary.csv created successfully.");
}
