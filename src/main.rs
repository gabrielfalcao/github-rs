use github_rs_config::Config;
use github_rs_client::Client;

fn main() {
    let config = Config::load("/Users/gabrielfalcao/.config/github-rs/config.yml");
    println!("Config {:?}", config);
    let client = Client::new(config);
    println!("Client {:?}", client);
}
