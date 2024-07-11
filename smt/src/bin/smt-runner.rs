use {
    clap::{value_t_or_exit, App, AppSettings, Arg, SubCommand},
    solana_smt::{
        model::{PostgresConfig, SMTerError},
        smter::*,
    },
    std::{fs::File, io::Read, path::PathBuf, process::exit, thread::Result},
};

fn main() -> Result<()> {
    let default_end_slot = u64::MAX.to_string();
    let max_tx_len = usize::MAX.to_string();
    let update_slot_arg = Arg::with_name("update_slot")
        .long("update-slot")
        .short("s")
        .value_name("UPDATE")
        .default_value(&default_end_slot)
        .help("Update SMT start from local DIR ledger slot to this slot");
    let query_slot_arg = Arg::with_name("query_slot")
        .long("query-slot")
        .short("s")
        .value_name("QUERY")
        .help("Query state from SMT at this slot");
    let query_tx_no_arg = Arg::with_name("query_tx")
        .long("query-tx")
        .short("t")
        .value_name("TX_NO")
        .default_value(&max_tx_len)
        .help("Query state from SMT at i-th transaction in QUERY slot");

    let matches = App::new("smt-runner")
        .about("SMT Runner")
        .version("0.1")
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("config_file")
                .short("c")
                .long("config")
                .value_name("CONFIG")
                .takes_value(true)
                .required(true)
                .default_value("config.json")
                .help("Configuration file to use"),
        )
        .arg(
            Arg::with_name("ledger_path")
                .short("l")
                .long("ledger")
                .value_name("DIR")
                .takes_value(true)
                .required(true)
                .default_value("ledger")
                .help("Use DIR as ledger location"),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Update SMT to SLOT slot")
                .arg(update_slot_arg),
        )
        .subcommand(
            SubCommand::with_name("query")
                .about("Query state from SMT for TX transaction at QUERY slot")
                .arg(&query_slot_arg)
                .arg(&query_tx_no_arg),
        )
        .get_matches();

    let config_file = value_t_or_exit!(matches, "config_file", PathBuf);
    let ledger_path = value_t_or_exit!(matches, "ledger_path", PathBuf);

    let mut file = File::open(config_file.as_path()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let config: PostgresConfig = serde_json::from_str(&contents)
        .map_err(|err| SMTerError::ConfigFileReadError {
            msg: format!(
                "The config file is not in the JSON format expected: {:?}",
                err
            ),
        })
        .unwrap();

    match matches.subcommand() {
        ("update", Some(arg_matches)) => {
            let update_slot = value_t_or_exit!(arg_matches, "update_slot", u64);
            match SMTer::update_smt(&config, &ledger_path, update_slot) {
                Ok(s) => {
                    let serialised = serde_json::to_string(&s).unwrap();
                    println!("Updated SMT, the summary are: {:?}", serialised);
                }
                Err(e) => {
                    println!("Failed to fetch db with err: {:?}", e);
                }
            }
        }
        ("query", Some(arg_matches)) => {
            let query_slot = value_t_or_exit!(arg_matches, "query_slot", u64);
            let query_tx_no = value_t_or_exit!(arg_matches, "query_tx", usize);

            match SMTer::query_tx_context(&config, &ledger_path, query_slot, query_tx_no) {
                Ok(tc) => {
                    let serialised = serde_json::to_string(&tc);
                    println!(
                        "Context for {}-th tx at slot {} are: {}",
                        query_tx_no,
                        query_slot,
                        serialised.unwrap()
                    );
                }
                Err(err) => {
                    println!("Failed to query TX context with err: {:?}", err);
                }
            }
        }
        ("", _) => {
            eprintln!("{}", matches.usage());
            exit(1);
        }
        _ => unreachable!(),
    }

    Ok(())
}
