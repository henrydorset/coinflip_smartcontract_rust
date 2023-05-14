use std::str::FromStr;
// use tracing_subscriber;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use secp256k1::SecretKey;
mod contract_queries;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Setup web3 clients and contract interfaces
    let transport = web3::transports::Http::new("http://127.0.0.1:7545").unwrap();
    let web3 = web3::Web3::new(transport);
    let coinflip_contract = contract_queries::CoinflipContract::new(
        &web3,
        // Update to match the deployed address
        "0x107949C4Bbe48cF3b23559a069fDf8Cee4976543".to_string(),
    )
    .await;

    // Create wallet
    let wallet: SecretKey = SecretKey::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();

    // Create list of features to select from
    let selections = &[
        "Create Match",
        "Join Match",
        "Query Match Info",
        "Get Total Matches",
        "Get Lifetime Value of Contract",
        "Exit",
    ];

    loop {
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => {
                // Create Match
                let bet = dialoguer::Input::<u128>::new()
                    .with_prompt("Enter Bet Amount (in wei)")
                    .interact()
                    .unwrap();

                coinflip_contract.create_match(wallet, bet).await;
            }
            1 => {
                // Join Match
                let match_id = dialoguer::Input::<u128>::new()
                    .with_prompt("Enter Match ID")
                    .interact()
                    .unwrap();
                let bet_amount = dialoguer::Input::<u128>::new()
                    .with_prompt("Enter Bet Amount")
                    .interact()
                    .unwrap();
                coinflip_contract
                    .join_match(wallet, match_id, bet_amount)
                    .await;
            }
            2 => {
                // Query Match Info
                let match_id = dialoguer::Input::<u128>::new()
                    .with_prompt("Enter Match ID")
                    .interact()
                    .unwrap();
                let match_info = coinflip_contract.query_match_info(match_id).await;
                println!("Match Info: {}", match_info);
            }
            3 => {
                // Get Total Matches
                let total_matches = coinflip_contract.get_total_matches().await;
                println!("Total Matches: {}", total_matches);
            }
            4 => {
                // Get Lifetime Value of Contract
                let lifetime_value = coinflip_contract.get_lifetime_value().await;
                println!("Lifetime Value: {} Wei", lifetime_value);
            }
            5 => {
                // Exit
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid selection");
            }
        }
    }

    Ok(())
}
