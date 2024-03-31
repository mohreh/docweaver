use docweaver::configuration::get_configuration;
use docweaver::App;

#[tokio::main]
async fn main() {
    let app: App = get_configuration()
        .expect("failed to read configuration")
        .into();

    app.run().await.expect("error running application");
}
