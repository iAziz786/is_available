use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let crate_name = std::env::args().nth(1).expect("no package name provided");

    let url_to_ping = format!("https://crates.io/api/v1/crates/{crate_name}", crate_name = crate_name);

    let client = reqwest::Client::new();
    let response = client
        .get(url_to_ping)
        .header("user-agent", "Mozilla/5.0")
        .send()
        .await?;

    let status = response.status().as_u16();

    if status != 200 {
        println!("available 👍");
    } else {
        println!("taken 👎");
    }

    Ok(())
}
