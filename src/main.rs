use clap::Parser;

mod command;
mod config;

fn main() {
	let cmd = command::Command::parse();
	println!("Command result {:#?}", cmd);

	let config = config::Config::default();
	println!("Base config:\n{}", serde_json::to_string_pretty(&config).unwrap());

	let merged_config = command::apply_command_config(config, cmd);
	println!("New config:\n{}", serde_json::to_string_pretty(&merged_config).unwrap());
}
