extern crate ananke;

use scheduler::Scheduler;

mod mfe;
mod cli;
mod task_runner;
mod scheduler;
mod configuration;

fn main() {
    let cli_options = cli::parse();

    let (
        microfrontends,
        configuration,
    ) = cli::link_options_adapter(cli_options);

    microfrontends.log();

    Scheduler::schedule(microfrontends.create_fetch_projects_info_tasks(&configuration.target_host, configuration.pull));
    Scheduler::schedule(microfrontends.create_install_dependency_tasks(configuration.force_update_all));
    Scheduler::schedule(microfrontends.create_run_tasks());

    // Error handling & logging
}
