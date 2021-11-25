use github_rs::Client;

// /Users/gabrielfalcao/.config/github-rs/config.yml
fn main() {
    let client = Client::new("foobar");
    println!("Client {:?}", client);
}
