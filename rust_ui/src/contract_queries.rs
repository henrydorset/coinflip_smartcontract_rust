use secp256k1::SecretKey;
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::{
    contract::{Contract, Options},
    types::{Address, U256},
};

use std::fmt;
use std::str::FromStr;

// newtype to better manage contract function handling.
pub struct CoinflipContract(Contract<Http>);

impl CoinflipContract {
    pub async fn new(web3: &web3::Web3<web3::transports::Http>, address: String,) -> Self {
        let address = Address::from_str(&address).unwrap();
        let contract =
            Contract::from_json(web3.eth(), address, include_bytes!("coinflip_abi.json")).unwrap();
        CoinflipContract(contract)
    }

    pub async fn get_total_matches(&self) -> u128 {
        let result: u128 = self.0
            .query("totalMatches", (), None, Options::default(), None)
            .await
            .unwrap();
        result
    }

    pub async fn get_lifetime_value(&self) -> U256 {
        let result: U256 = self
            .0
            .query("lifetimeValue", (), None, Options::default(), None)
            .await
            .unwrap();
        result
    }

    pub async fn query_match_info(&self, match_id: u128) -> Match {
        let result: (Address, Address, u128, u128, bool, u128) = self
            .0
            .query("matches", match_id, None, Options::default(), None)
            .await
            .unwrap();

        Match {
            match_id: match_id,
            player1: result.0,
            player2: result.1,
            player1_bet: result.2,
            player2_bet: result.3,
            match_complete: result.4,
            winner: result.5,
        }
    }

    pub async fn create_match(&self, account: SecretKey, bet: u128) {
        // Signed call to create the transaction
        let tx = self
            .0

            .signed_call(
                "createMatch",
                (),
                Options {
                    gas: Some(5_000_000.into()),
                    value: Some(bet.into()),
                    ..Default::default()
                },
                SecretKeyRef::new(&account),
            )
            .await
            .unwrap();

        // Query total number of matches -1 to to get the id of the match we just created
        let match_id: u128 = self
            .0
            .query("totalMatches", (), None, Options::default(), None)
            .await
            .unwrap();

        // Get the information on the new match
        let match_info = self.query_match_info(match_id - 1).await;

        println!(
            r#"
Match created! 
Transaction ID: {}
___________________
{}
        "#,
            tx, match_info
        )
    }

    pub async fn join_match(&self, account: SecretKey, match_id: u128, bet: u128) {
        // Signed call to create the transaction
        println!("Joining match {} with bet of {} wei", match_id, bet);
        let tx = self
            .0
            // It will attempt to dereference the key into a secp256k1::key::SecretKey, so it needs to be a SecretKeyRef
            .signed_call(
                "joinMatch",
                match_id, // Joins the match of the users choice
                Options {
                    gas: Some(5_000_000.into()),
                    value: Some(bet.into()), // Bet must be equal to the value of opposing user in the match
                    ..Default::default()
                },
                SecretKeyRef::new(&account),
            )
            .await
            .unwrap();

        // Get the information on the new match
        let match_info = self.query_match_info(match_id).await;
        println!(
            r#"
Match completed!
Transaction ID: {}
Player {} wins!
Full match details:
___________________
{}
"#,
            tx, match_info.winner, match_info
        )
    }
}

pub struct Match {
    match_id: u128,
    player1: Address,
    player2: Address,
    player1_bet: u128,
    player2_bet: u128,
    winner: u128,
    match_complete: bool,
}

impl fmt::Display for Match {
    //Display formatter for the given banner entry
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Bet values are being output in Wei by default. Must conver them later
        write!(
            f,
            "
Match ID: {}{}
Player 1: {}: {} Wei 
Player 2: {}: {} Wei
Completed: {}
Winner: Player {}",
            "",
            self.match_id,
            self.player1,
            self.player1_bet,
            self.player2,
            self.player2_bet,
            self.match_complete,
            self.winner
        )
    }
}
