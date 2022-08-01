use crate::config::*;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Command {
	// Lighting
	/// LED lighting mode
	#[clap(long, value_parser)]
	pub mode: Option<lighting::Mode>,

	/// LED animation speed in Rainbow mode (1-3)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=3))]
	pub rainbow_speed: Option<u8>,
	/// LED animation direction in Rainbow mode
	#[clap(long, value_parser)]
	pub rainbow_direction: Option<lighting::RainbowDirection>,

	/// LED brightness in Solid mode (1-4)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=4))]
	pub solid_brightness: Option<u8>,
	/// LED color in Solid mode (hex)
	#[clap(long, value_parser = color_parser)]
	pub solid_color: Option<Color>,

	/// LED brightness in Breathing mode (1-4)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=4))]
	pub breathing_brightness: Option<u8>,
	/// LED animation speed in Breathing mode (1-3)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=3))]
	pub breathing_speed: Option<u8>,
	/// Set Breathing color (<index 0-6>:<hex color>)
	#[clap(long, value_parser = idx_split_parse::<Color, 0, 6>)]
	pub breathing_color: Vec<(u8, Color)>,

	/// LED brightness in Tail mode (1-4)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=4))]
	pub tail_brightness: Option<u8>,
	/// LED animation speed in Tail mode (1-3)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=3))]
	pub tail_speed: Option<u8>,

	/// LED animation speed in Fade mode (1-3)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=3))]
	pub fade_speed: Option<u8>,

	/// LED brightness in Rave mode (1-4)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=4))]
	pub rave_brightness: Option<u8>,
	/// LED animation speed in Rave mode (1-3)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=3))]
	pub rave_speed: Option<u8>,
	/// Set Rave color (<index 0-1>:<hex color>)
	#[clap(long, value_parser = idx_split_parse::<Color, 0, 1>)]
	pub rave_color: Vec<(u8, Color)>,

	/// LED brightness in Wave mode (1-4)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=4))]
	pub wave_brightness: Option<u8>,
	/// LED animation speed in Wave mode (1-3)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=3))]
	pub wave_speed: Option<u8>,

	/// LED animation speed in Breathing (Single) mode (1-3)
	#[clap(long, value_parser = clap::value_parser!(u8).range(1..=3))]
	pub breathing_single_speed: Option<u8>,
	/// LED color in Solid mode (hex)
	#[clap(long, value_parser = color_parser)]
	pub breathing_single_color: Option<Color>,

	// Dpi
	/// Enable a DPI setting (0-5)
	#[clap(long, value_parser = clap::value_parser!(u8).range(0..=5))]
	pub enable_dpi: Vec<u8>,
	/// Disable a DPI setting (0-5)
	#[clap(long, value_parser = clap::value_parser!(u8).range(0..=5))]
	pub disable_dpi: Vec<u8>,
	/// Toggle a DPI setting (0-5)
	#[clap(long, value_parser = clap::value_parser!(u8).range(0..=5))]
	pub toggle_dpi: Vec<u8>,
	/// Set the color for a DPI setting (<index 0-5>:<hex color>))
	#[clap(long, value_parser = idx_split_parse::<Color, 0, 5>)]
	pub dpi_color: Vec<(u8, Color)>,
	/// Set the X and Y DPI for a DPI setting (<index 0-5>:<dpi ending in 00>)
	#[clap(long, value_parser = idx_split_parse_cast::<DpiWrapper, u8, 0, 5>)]
	pub dpi: Vec<(u8, u8)>,
	/// Set the X DPI for a DPI setting (<index 0-5>:<dpi ending in 00>)
	#[clap(long, value_parser = idx_split_parse_cast::<DpiWrapper, u8, 0, 5>)]
	pub dpi_x: Vec<(u8, u8)>,
	/// Set the Y DPI for a DPI setting (<index 0-5>:<dpi ending in 00>)
	#[clap(long, value_parser = idx_split_parse_cast::<DpiWrapper, u8, 0, 5>)]
	pub dpi_y: Vec<(u8, u8)>,
	/// Reset dpis not listed
	#[clap(long, value_parser)]
	pub reset_dpis: bool,
	/// Set current DPI
	#[clap(long, value_parser = clap::value_parser!(u8).range(0..=5))]
	pub select_dpi: Option<u8>,

	/// Set polling rate
	#[clap(long, value_parser)]
	pub polling_rate: Option<PollingRate>,

	/// Set liftoff distance (millimeters)
	#[clap(long, value_parser)]
	pub liftoff_distance: Option<LiftoffDistance>,
}

/// Newtype struct used to format a polling rate value
struct DpiWrapper(u8);

impl TryFrom<&str> for DpiWrapper {
	type Error = String;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		if !value.ends_with("00") {
			Err(r#"dpi did not end in "00""#.to_owned())
		} else {
			Ok(Self(
				u8::from_str(&value[..value.len() - 2])
					.map_err(|_| "dpi was not between 100 and 25500")?,
			))
		}
	}
}

impl From<DpiWrapper> for u8 {
	fn from(dpi: DpiWrapper) -> Self {
		dpi.0
	}
}

/// Function to parse a color from a hex string, returns
/// the value of [`TryFrom`]`<&str>` for [`Color`].
///
/// # Errors
///
/// If the string is not a 6 digit hexadecimal color
fn color_parser(s: &str) -> Result<Color, String> {
	TryFrom::try_from(s)
}

/// Split a string in the form `<index>:<value>` into a `u8` index
/// and `T` value.
///
/// `T` must implement [`TryFrom`]`<&str, Error = String>` so
/// `<value>` can be converted to a `T`.
///
/// `<index>` must be in the range `MIN_IDX..=MAX_IDX`.
///
/// # Errors
///
/// An error will be returned if the value of `<index>`
/// is not in `MIN_RANGE..=MAX_RANGE`, or the [`TryFrom`]
/// implementation for `<value>` returns an error.
fn idx_split_parse<T, const MIN_IDX: u8, const MAX_IDX: u8>(s: &str) -> Result<(u8, T), String>
where
	T: for<'a> TryFrom<&'a str, Error = String>,
{
	let parts = s.split(':').collect::<Vec<&str>>();

	if parts.len() != 2 {
		Err(r#"must be in the form "<index>:<value>""#.to_owned())
	} else {
		let (idx_str, value_str) = (parts[0], parts[1]);
		let idx = u8::from_str(idx_str)
			.ok()
			.filter(|idx| (MIN_IDX..=MAX_IDX).contains(idx))
			.ok_or(format!("<idx> must be in range {MIN_IDX}..{MAX_IDX}"))?;
		let color = T::try_from(value_str)?;

		Ok((idx, color))
	}
}

/// Split a string in the form `<index>:<value>` into a `u8` index
/// and `T` value, then convert that `T` value into a `C` value.
///
/// `T` must implement [`TryFrom`]`<&str, Error = String>` so
/// `<value>` can be converted to a `T`.
///
/// `<index>` must be in the range `MIN_IDX..=MAX_IDX`.
///
/// `T` must also implement `Into<C>` so `<value>: T` can be
/// converted to a `C` value and returned.
///
/// # Errors
///
/// An error will be returned if the value of `<index>`
/// is not in `MIN_RANGE..=MAX_RANGE`, or the [`TryFrom`]
/// implementation for `<value>` returns an error.
fn idx_split_parse_cast<T, C, const MIN_IDX: u8, const MAX_IDX: u8>(
	s: &str,
) -> Result<(u8, C), String>
where
	T: for<'a> TryFrom<&'a str, Error = String> + Into<C>,
{
	idx_split_parse::<T, MIN_IDX, MAX_IDX>(s).map(|(idx, val)| (idx, Into::into(val)))
}

/// Merges values of `HashMap<(u8, T)>` into a `[T; N]`,
/// where the first element is an index, and the second
/// is a value, returning a `[T; N]` with the values of `vec`
/// where they are present.
fn merge_map<T, const N: usize>(mut array: [T; N], mut map: HashMap<u8, T>) -> [T; N] {
	for (i, element) in array.iter_mut().enumerate() {
		if let Some(new_value) = map.remove(&(i as u8)) {
			*element = new_value;
		}
	}
	array
}

/// Apply the values of a [`Command`] struct to a [`Config`] struct,
/// returning the [`Config`] struct.
pub fn apply_command_config(config: Config, command: Command) -> Config {
	Config {
		lighting: lighting::Lighting {
			mode: command.mode.unwrap_or(config.lighting.mode),
			solid: lighting::Solid {
				brightness: command
					.solid_brightness
					.map(RangedByte)
					.unwrap_or(config.lighting.solid.brightness),
				color: command.solid_color.unwrap_or(config.lighting.solid.color),
			},
			rainbow: lighting::Rainbow {
				speed: command
					.rainbow_speed
					.map(RangedByte)
					.unwrap_or(config.lighting.rainbow.speed),
				direction: command
					.rainbow_direction
					.unwrap_or(config.lighting.rainbow.direction),
			},
			breathing: lighting::Breathing {
				brightness: command
					.breathing_brightness
					.map(RangedByte)
					.unwrap_or(config.lighting.breathing.brightness),
				speed: command
					.breathing_speed
					.map(RangedByte)
					.unwrap_or(config.lighting.breathing.speed),
				colors: merge_map(
					config.lighting.breathing.colors,
					command.breathing_color.into_iter().collect(),
				),
			},
			tail: lighting::Tail {
				brightness: command
					.tail_brightness
					.map(RangedByte)
					.unwrap_or(config.lighting.tail.brightness),
				speed: command
					.tail_speed
					.map(RangedByte)
					.unwrap_or(config.lighting.tail.speed),
			},
			fade: lighting::Fade {
				speed: command
					.fade_speed
					.map(RangedByte)
					.unwrap_or(config.lighting.fade.speed),
			},
			rave: lighting::Rave {
				brightness: command
					.rave_brightness
					.map(RangedByte)
					.unwrap_or(config.lighting.rave.brightness),
				speed: command
					.rave_speed
					.map(RangedByte)
					.unwrap_or(config.lighting.rave.speed),
				colors: merge_map(
					config.lighting.rave.colors,
					command.rave_color.into_iter().collect(),
				),
			},
			wave: lighting::Wave {
				brightness: command
					.wave_brightness
					.map(RangedByte)
					.unwrap_or(config.lighting.wave.brightness),
				speed: command
					.wave_speed
					.map(RangedByte)
					.unwrap_or(config.lighting.wave.speed),
			},
			breathing_single: lighting::BreathingSingle {
				speed: command
					.breathing_single_speed
					.map(RangedByte)
					.unwrap_or(config.lighting.breathing_single.speed),
				color: command
					.breathing_single_color
					.unwrap_or(config.lighting.breathing_single.color),
			},
		},
		dpi: {
			let dpi_enable_overrides = {
				let toggle_map = command.toggle_dpi.into_iter().collect::<HashSet<u8>>();
				let enable_map = command.enable_dpi.into_iter().collect::<HashSet<u8>>();
				let disable_map = command.disable_dpi.into_iter().collect::<HashSet<u8>>();

				(0..=5)
					.map(|i| {
						(
							i,
							if toggle_map.contains(&i) {
								Some(!config.dpi[i as usize].enable)
							} else if enable_map.contains(&i) {
								Some(true)
							} else if disable_map.contains(&i) || command.reset_dpis {
								Some(false)
							} else {
								None
							},
						)
					})
					.filter_map(|(i, state)| state.map(|state| (i, state)))
					.collect::<HashMap<u8, bool>>()
			};

			let mut dpi_color_overrides = command
				.dpi_color
				.into_iter()
				.collect::<HashMap<u8, Color>>();
			let base_dpi_overrides = command.dpi.into_iter().collect::<HashMap<u8, u8>>();
			let x_dpi_overrides = command.dpi_x.into_iter().collect::<HashMap<u8, u8>>();
			let y_dpi_overrides = command.dpi_y.into_iter().collect::<HashMap<u8, u8>>();

			let dpi_overrides = (0..=5)
				.map(|i| {
					(
						i,
						Dpi {
							enable: match dpi_enable_overrides.get(&i) {
								Some(state) => *state,
								None => config.dpi[i as usize].enable,
							},
							color: match dpi_color_overrides.remove(&i) {
								Some(color) => color,
								None => config.dpi[i as usize].color.clone(),
							},
							x_dpi: match x_dpi_overrides.get(&i) {
								Some(x_dpi) => *x_dpi,
								None => match base_dpi_overrides.get(&i) {
									Some(base_dpi) => *base_dpi,
									None => config.dpi[i as usize].x_dpi,
								},
							},
							y_dpi: match y_dpi_overrides.get(&i) {
								Some(y_dpi) => *y_dpi,
								None => match base_dpi_overrides.get(&i) {
									Some(base_dpi) => *base_dpi,
									None => config.dpi[i as usize].y_dpi,
								},
							},
						},
					)
				})
				.collect::<HashMap<u8, Dpi>>();

			merge_map(config.dpi, dpi_overrides)
		},
		current_dpi: command
			.select_dpi
			.map(RangedByte)
			.unwrap_or(config.current_dpi),
		polling_rate: command.polling_rate.unwrap_or(config.polling_rate),
		liftoff_distance: command.liftoff_distance.unwrap_or(config.liftoff_distance),
	}
}
