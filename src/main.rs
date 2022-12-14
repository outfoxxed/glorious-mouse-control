use std::fs;

use clap::Parser;

mod command;
mod config;
mod usb;

/// Exit the process nicely
macro_rules! error {
	($($arg:tt)*) => {{
		eprintln!("error: {}", format!($($arg)*));
		::std::process::exit(1)
	}}
}
pub(crate) use error;

fn main() {
	let cmd = command::Command::parse();

	// must be destructured to avoid the lifetime disallowing `apply_command_config`
	let command::ExtraFlags {
		save_config,
		use_config,
		config_location,
	} = cmd.flags();

	let config_path = config_location
		.map(|p| std::path::PathBuf::from(p))
		.unwrap_or_else(|| {
			xdg::BaseDirectories::with_prefix("glorious-mouse-control")
				.unwrap_or_else(|e| error!("error getting XDG directories: {e}"))
				.place_config_file("config.json")
				.unwrap_or_else(|e| error!("could not create config file: {e}"))
		});

	let mut merged_config = cmd.apply_command_config(if use_config {
		match fs::read_to_string(&config_path).ok() {
			Some(config_json) => serde_json::from_str::<config::Config>(&config_json)
				.unwrap_or_else(|e| error!("could not parse config file: {e}")),
			None => config::Config::default(),
		}
	} else {
		config::Config::default()
	});

	// at least one DPI must be enabled
	if !merged_config
		.dpi
		.iter()
		.any(|config::Dpi { enable, .. }| *enable)
	{
		error!("at least one DPI must be enabled")
	}

	// check that the selected dpi is enabled and reset if it isn't
	if !merged_config.dpi[*merged_config.current_dpi as usize].enable {
		merged_config.current_dpi = config::RangedByte(
			merged_config
				.dpi
				.iter()
				.enumerate()
				.find(|(_, config::Dpi { enable, .. })| *enable)
				// unwrap will never panic because at least one dpi must be enabled, as asserted above
				.unwrap()
				.0 as u8,
		);

		eprintln!(
			"warning: the selected DPI is not enabled - DPI {} has been selected instead",
			*merged_config.current_dpi
		);
	}

	if save_config {
		fs::write(
			&config_path,
			serde_json::to_string_pretty(&merged_config).unwrap_or_else(|e| {
				error!("could not create a json representation of the current config: {e}")
			}),
		)
		.unwrap_or_else(|e| error!("could not save config file: {e}"));
	}

	usb::apply_config(&merged_config);

	println!("new configuration applied");
}
