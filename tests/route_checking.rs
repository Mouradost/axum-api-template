use std::collections::HashMap;

use colored::*;
use reqwest::Response;

// cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
#[tokio::test]
async fn route_check() -> anyhow::Result<()> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let res = client
        .get("http://localhost:8080/Cargo.toml")
        .send()
        .await?;
    display_response("Requesting a file", res).await?;

    let mut user_cred = HashMap::new();
    user_cred.insert("username", "test");
    user_cred.insert("password", "test");
    let res = client
        .post("http://localhost:8080/api/register")
        .json(&user_cred)
        .send()
        .await?;
    display_response("Register", res).await?;

    let res = client
        .post("http://localhost:8080/api/login")
        .json(&user_cred)
        .send()
        .await?;
    display_response("Login", res).await?;

    let res = client
        .post("http://localhost:8080/api/logout")
        .send()
        .await?;
    display_response("Logout", res).await?;

    let res = client
        .post("http://localhost:8080/api/logout")
        .send()
        .await?;
    display_response("Logout", res).await?;
    Ok(())
}

async fn display_response(title: &str, res: Response) -> anyhow::Result<()> {
    println!("{}", "----------------------------------".red());
    println!("{}\n\n{:#?}", title.blue(), res);
    println!("{}", "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~".yellow());
    println!("{}\n\n{}", "Content".green(), res.text().await?);
    println!("{}", "----------------------------------".red());
    Ok(())
}
