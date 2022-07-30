use clap::Parser;
use std::fs;

mod command;
mod config;

macro_rules! error {
	($($arg:tt)*) => {{
		eprintln!("error: {}", format!($($arg)*));
		::std::process::exit(1)
	}}
}

fn main() {
	let cmd = command::Command::parse();

	let config_path = xdg::BaseDirectories::with_prefix("glorious-mouse-control")
		.unwrap_or_else(|e| error!("error getting XDG directories: {}", e))
		.place_config_file("config.json")
		.unwrap_or_else(|e| error!("could not create config file: {}", e));

	let config = match fs::read_to_string(&config_path).ok() {
		Some(config_json) => serde_json::from_str::<config::Config>(&config_json)
			.unwrap_or_else(|e| error!("could not parse config file: {}", e)),
		None => config::Config::default(),
	};

	let merged_config = command::apply_command_config(config, cmd);

	fs::write(
		&config_path,
		serde_json::to_string_pretty(&merged_config).unwrap_or_else(|e| {
			error!("could not create a json representation of the current config: {}", e)
		}),
	)
	.unwrap_or_else(|e| error!("could not save config file: {}", e));
}
