use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello-world").await?.print().await?;
    hc.do_get("/hello-world?name=Andrew").await?.print().await?;
    hc.do_get("/hello-world/Andrew").await?.print().await?;

    Ok(())
}
