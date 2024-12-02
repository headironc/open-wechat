use open_wechat_helper::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let application = Application::new().await?;

    application.run().await?;

    Ok(())
}
