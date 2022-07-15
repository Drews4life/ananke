use clap::{Arg, value_parser, Command};
use clap::parser::ArgMatches;
use crate::mfe::{Microfrontend, Microfrontends};
use crate::configuration::Configuration;

mod options {
    pub mod short {
        pub const MFE: char = 'm';
        pub const TARGET_HOST: char = 't';
        pub const FORCE_UPDATE_ALL: char = 'f';
        pub const PULL: char = 'p';
    }

    pub mod long {
        pub const MFE: &str = "microfrontends";
        pub const TARGET_HOST: &str = "target-host";
        pub const FORCE_UPDATE_ALL: &str = "force-update-all";
        pub const PULL: &str = "pull";
    }
}

pub struct LinkCliResult {
    pub target_host: String,
    pub microfrontend: Vec<String>,
    pub force_update_all: bool,
    pub pull: bool,
}

fn initiate() -> Command<'static> {
    Command::new("ananke")
        .about("CLI betbook microfrontend linker ❤️")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("link")
                .about("Link microfrontends and run altogether")
                .args(
                    vec![
                        Arg::with_name(options::long::MFE)
                            .short(options::short::MFE)
                            .long(options::long::MFE)
                            .help("Which microfrontends to fetch")
                            .takes_value(true)
                            .required(true)
                            .multiple_values(true)
                            .value_parser(value_parser!(String)),
                        Arg::with_name(options::long::TARGET_HOST)
                            .short(options::short::TARGET_HOST)
                            .long(options::long::TARGET_HOST)
                            .help("Where repo is fetched from")
                            .takes_value(true)
                            .required(true)
                            .value_parser(value_parser!(String)),
                        Arg::with_name(options::long::FORCE_UPDATE_ALL)
                            .short(options::short::FORCE_UPDATE_ALL)
                            .long(options::long::FORCE_UPDATE_ALL)
                            .help("Re-install dependencies for all MFEs")
                            .takes_value(false)
                            .required(false),
                        Arg::with_name(options::long::PULL)
                            .short(options::short::PULL)
                            .long(options::long::PULL)
                            .help("Pull latest changes from desired versions")
                            .takes_value(false)
                            .required(false),
                        //  TODO: arg!(--"force-update" <FORCE UPDATE DESIRED MFE>).required(false),
                    ],
                ),
        )
}

fn parse_link_command(arg_matches: &ArgMatches) -> LinkCliResult {
    let target_host = arg_matches.get_one::<String>(options::long::TARGET_HOST).unwrap().to_string();
    let microfrontend: Vec<_> = arg_matches.values_of(options::long::MFE).unwrap()
        .map(|str| (*str).to_owned())
        .collect();
    let force_update_all = arg_matches.is_present(options::long::FORCE_UPDATE_ALL);
    let pull = arg_matches.is_present(options::long::PULL);

    LinkCliResult { target_host, microfrontend, force_update_all, pull }
}

pub fn parse() -> LinkCliResult {
    let matches = self::initiate().get_matches();
    let arg_matches = matches.subcommand_matches("link").unwrap();

    self::parse_link_command(arg_matches)
}

pub fn link_options_adapter(link_result: LinkCliResult) -> (Microfrontends, Configuration) {
    let configuration = Configuration {
        target_host: link_result.target_host,
        force_update_all: link_result.force_update_all,
        pull: link_result.pull
    };
    let microfrontends_collection = link_result.microfrontend
        .iter()
        .map(|mfe| Microfrontend::from_raw_format(mfe.clone()))
        .collect();
    let mfes = Microfrontends(microfrontends_collection);

    (mfes, configuration)
}