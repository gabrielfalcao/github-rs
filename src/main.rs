use github_rs_client::Client;
use github_rs_config::Config;

fn main() {
    let config = Config::load("/Users/gabrielfalcao/.config/github-rs/config.yml");

    let client = Client::new(config);

    client.get_user();
}
