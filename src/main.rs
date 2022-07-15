extern crate ananke;

use scheduler::Scheduler;
use tokio;
use tokio::io::AsyncWriteExt;
use crate::configuration::Configuration;

mod mfe;
mod cli;
mod task_runner;
mod scheduler;
mod configuration;

#[tokio::main]
async fn main() {
    let cli_options = cli::parse();

    let (
        microfrontends,
        configuration,
    ) = cli::link_options_adapter(cli_options);

    microfrontends
        .into_iter()
        .map(|microfrontend| {
            // Temporal workaround
            let config = configuration.clone();
            tokio::spawn(async move {
                println!("Fetching: {:?}", microfrontend);
                microfrontend.init(&config);
            })
        })
        .for_each(|h| {
            async {
                h.await.unwrap();
            };
        });
}
