use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Configuration
    let mut conf = Configuration::new();
		conf.bearer_access_token = Some("".to_string());

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

    Ok(())
}