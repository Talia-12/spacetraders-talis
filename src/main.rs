use std::fs;
use std::time::Duration;
use serde_json::json;
use spacedust::apis::Error;
use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::fleet_api::{extract_resources, get_my_ships, ExtractResourcesError};
use spacedust::apis::configuration::Configuration;
use spacedust::models::ExtractResourcesRequest;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
		// Create Configuration
		let mut conf = Configuration::new();

		let contents = fs::read_to_string("token.txt")
				.expect("Should have been able to read the file");

		conf.bearer_access_token = Some(contents);

		// Get Agent Details to Confirm Working
		match get_my_agent(&conf).await {
				Ok(res) => {
					println!("{:#?}", res);
					// Print Symbol
					println!("My Symbol: {:#?}", res.data.symbol);
				}
				Err(err_res) => {
						panic!("{:#?}", err_res);
				}
		}

		match get_my_ships(&conf, None, None).await {
			Ok(res) => {
				// Print Symbol
				for ship in res.data {
					println!("Ship Symbol: {:#?}", ship.symbol);
				}

			}
			Err(err_res) => {
					panic!("{:#?}", err_res);
			}
		}

		let mut new_delay: u64;
		loop {
			match extract_resources(&conf, "TALIA12-3", Some(ExtractResourcesRequest::new())).await {
				Ok(res) => {
					// Print Cargo
					println!("{:#?}", res.data.cargo);

					new_delay = res.data.cooldown.remaining_seconds as u64 + 1;

					println!("Delaying for {:#?} seconds.", new_delay);
				}
				Err(err_res) => {
						match err_res {
							Error::ResponseError(err_res) => {
								let extract_err = err_res.entity.as_ref().unwrap();
								
								match extract_err {
									ExtractResourcesError::UnknownValue(value) => {
										if let Some(error) = value.get("error") {
											if error.get("code") == Some(&json!(4000)) {
												
												new_delay = error["data"]["cooldown"]["remainingSeconds"].as_u64().unwrap() + 1;

												println!("Called while still on cooldown, delaying for {:#?} remaining seconds.", new_delay);

											} else {
												panic!("{:#?}", err_res)
											}
										} else {
											panic!("{:#?}", err_res)
										}
									},
								}
							},
							_ => panic!("{:#?}", err_res)
						}
				}
			}

			sleep(Duration::from_secs(new_delay)).await;
		}
}