mod config;

fn main() {
	let cfg_str = serde_json::to_string_pretty(&config::Config::default()).unwrap();
	println!("Default config:\n{}", &cfg_str);
}
