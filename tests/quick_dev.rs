use anyhow::Result;
use serde_json::json;

// cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
#[tokio::test]
async fn quick_dev() -> Result<()> {
    // Bind to the client on the server address
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Login
    // Incorrect cred
    // let req_login = hc.do_post(
    //     "/api/login",
    //     json!({"username": "test", "password": "test"}),
    // );
    // req_login.await?.print().await?;
    // let req_login = hc.do_post(
    //     "/api/register",
    //     json!({"username": "mourad", "password": "1234"}),
    // );
    // req_login.await?.print().await?;
    // Correct cred
    let req_login = hc.do_post(
        "/api/login",
        json!({"username": "mourad", "password": "1234"}),
    );
    req_login.await?.print().await?;
    // logout
    // Correct cred
    let req_logout = hc.do_post(
        "/api/logout",
        json!({"username": "mourad", "password": "1234"}),
    );
    req_logout.await?.print().await?;


    Ok(())
}
