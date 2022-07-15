extern crate ananke;

use scheduler::Scheduler;
use tokio;
use futures;
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

    let handlers = microfrontends
        .into_iter()
        .map(|microfrontend| {
            let config = configuration.clone();

            tokio::spawn(async move {
                println!("Fetching: {:?}", microfrontend);
                microfrontend.init(&config);
            })
        });

    futures::future::join_all(handlers).await;
}
