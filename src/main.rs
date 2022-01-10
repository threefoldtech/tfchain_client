use clap::{App, Arg};
use git_version::git_version;
use keyring::AccountKeyring;
use tfchain_client::AccountId32;

extern crate tokio;

const GIT_VERSION: &str = git_version!(args = ["--tags", "--always", "--dirty=-modified"]);

#[tokio::main]
async fn main() {
    let matches = App::new("tfchaincli")
        .author("ThreeFold Tech, https://github.com/threefoldtech")
        .version(GIT_VERSION)
        .about("A tfchain command line client")
        .arg(
            Arg::new("websocket")
                .value_name("WEBSOCKET")
                .short('s')
                .long("websocket")
                .default_value("wss://tfchain.dev.grid.tf")
                .help("substrate websocket connection"),
        )
        .subcommand(
            App::new("farms").about("Farm operations").subcommand(
                App::new("get").about("Get farm").arg(
                    Arg::new("farm_id")
                        .help("get by id")
                        .takes_value(true)
                        .required(true),
                ),
            ),
        )
        .subcommand(
            App::new("balance").about("Balance operations").subcommand(
                App::new("get").about("Get balance").arg(
                    Arg::new("account")
                        .help("the account for which to query the balance")
                        .takes_value(true)
                        .required(true),
                ),
            ),
        )
        .subcommand(
            App::new("node")
                .about("Get a node registered on chain")
                .subcommand(
                    App::new("get").about("Get Node by ID").arg(
                        Arg::new("node_id")
                            .help("the id of the node to fetch")
                            .takes_value(true)
                            .required(true),
                    ),
                ),
        )
        .subcommand(
            App::new("contract")
                .about("Get a contract registered on chain")
                .subcommand(
                    App::new("get").about("Get Contract by ID").arg(
                        Arg::new("contract_id")
                            .help("the id of the contract to fetch")
                            .takes_value(true)
                            .required(true),
                    ),
                ),
        )
        .subcommand(
            App::new("twin")
                .about("Get a twin registered on chain")
                .subcommand(
                    App::new("get").about("Get Twin by ID").arg(
                        Arg::new("twin_id")
                            .help("the id of the twin to fetch")
                            .takes_value(true)
                            .required(true),
                    ),
                ),
        )
        .get_matches();

    let websocket = matches.value_of("websocket").unwrap();

    match matches.subcommand() {
        Some(("farms", farm_matches)) => {
            if let Some(get_farm) = farm_matches.subcommand_matches("get") {
                match get_farm.value_of_t("farm_id") {
                    Ok(farm_id) => {
                        let from = AccountKeyring::Alice.pair();
                        let client = tfchain_client::Client::new(String::from(websocket), from);
                        let farm = client.get_farm_by_id(farm_id).unwrap();
                        println!("farm: {:?}", farm);
                    }
                    Err(e) => {
                        println!("could not find farm: {}", e);
                    }
                }
            }
        }
        Some(("balance", account)) => {
            if let Some(get_balance) = account.subcommand_matches("get") {
                let account = get_balance.value_of("account").unwrap();
                let from = AccountKeyring::Alice.pair();
                let client = tfchain_client::Client::new(String::from(websocket), from);
                match account.parse::<AccountId32>() {
                    Ok(ref account) => {
                        let balance = client.get_account_free_balance(account).unwrap();
                        println!("Free balance for account {}: {} TFT", account, balance);
                    }
                    Err(e) => {
                        println!("{} is not a valid account ({})", account, e);
                    }
                }
            }
        }
        Some(("node", node_data)) => {
            if let Some(get_node) = node_data.subcommand_matches("get") {
                let from = AccountKeyring::Alice.pair();
                let client = tfchain_client::Client::new(String::from(websocket), from);
                match get_node.value_of_t("node_id") {
                    Ok(node_id) => {
                        let node = client.get_node_by_id(node_id).unwrap();
                        println!("{}", node);
                    }
                    Err(e) => println!("could not parse node_id: {}", e),
                }
            }
        }
        Some(("contract", contract_data)) => {
            if let Some(get_contract) = contract_data.subcommand_matches("get") {
                let from = AccountKeyring::Alice.pair();
                let client = tfchain_client::Client::new(String::from(websocket), from);
                match get_contract.value_of_t("contract_id") {
                    Ok(contract_id) => {
                        let contract = client.get_contract_by_id(contract_id).unwrap();
                        println!("{}", contract);
                    }
                    Err(e) => println!("could not parse contract_id: {}", e),
                }
            }
        }
        Some(("twin", twin_data)) => {
            if let Some(get_twin) = twin_data.subcommand_matches("get") {
                let from = AccountKeyring::Alice.pair();
                let client = tfchain_client::Client::new(String::from(websocket), from);
                match get_twin.value_of_t("twin_id") {
                    Ok(twin_id) => {
                        let twin = client.get_twin_by_id(twin_id).unwrap();
                        println!("{}", twin);
                    }
                    Err(e) => println!("could not parse twin_id: {}", e),
                }
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    };
}
