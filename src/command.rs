use std::{
	collections::{HashMap, HashSet},
	str::FromStr,
};

use clap::Parser;

use crate::config::*;

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

	/// Set debounce time (ms)
	#[clap(long, value_parser)]
	pub debounce_time: Option<DebounceTime>,

	// Mouse buttons
	/// Left mouse button action
	#[clap(long, value_parser)]
	pub left_button: Option<MouseButtonType>,
	/// Left mouse button action
	#[clap(long, value_parser)]
	pub right_button: Option<MouseButtonType>,
	/// Left mouse button action
	#[clap(long, value_parser)]
	pub middle_button: Option<MouseButtonType>,
	/// Left mouse button action
	#[clap(long, value_parser)]
	pub forward_button: Option<MouseButtonType>,
	/// Left mouse button action
	#[clap(long, value_parser)]
	pub back_button: Option<MouseButtonType>,
	/// Left mouse button action
	#[clap(long, value_parser)]
	pub dpi_button: Option<MouseButtonType>,

	// Extra Flags
	/// Only use flags (ignore config file)
	#[clap(long, value_parser)]
	pub noconf: bool,
	/// Don't save config file (changes will reset next run)
	#[clap(long, value_parser)]
	pub nosave: bool,
	/// Set the config file location
	#[clap(long, value_parser)]
	pub config: Option<String>,
}

/// Extra CLI params that don't go in the config file
pub struct ExtraFlags<'c> {
	pub save_config: bool,
	pub use_config: bool,
	pub config_location: Option<&'c str>,
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

impl Command {
	/// Get extra flags that don't modify the config
	pub fn flags<'s>(&'s self) -> ExtraFlags<'s> {
		ExtraFlags {
			save_config: !self.nosave,
			use_config: !self.noconf,
			config_location: self.config.as_ref().map(|s| &**s),
		}
	}

	/// Apply the values of a [`Command`] struct to a [`Config`] struct,
	/// returning the [`Config`] struct.
	pub fn apply_command_config(self, config: Config) -> Config {
		Config {
			lighting: lighting::Lighting {
				mode: self.mode.unwrap_or(config.lighting.mode),
				solid: lighting::Solid {
					brightness: self
						.solid_brightness
						.map(RangedByte)
						.unwrap_or(config.lighting.solid.brightness),
					color: self.solid_color.unwrap_or(config.lighting.solid.color),
				},
				rainbow: lighting::Rainbow {
					speed: self
						.rainbow_speed
						.map(RangedByte)
						.unwrap_or(config.lighting.rainbow.speed),
					direction: self
						.rainbow_direction
						.unwrap_or(config.lighting.rainbow.direction),
				},
				breathing: lighting::Breathing {
					brightness: self
						.breathing_brightness
						.map(RangedByte)
						.unwrap_or(config.lighting.breathing.brightness),
					speed: self
						.breathing_speed
						.map(RangedByte)
						.unwrap_or(config.lighting.breathing.speed),
					colors: merge_map(
						config.lighting.breathing.colors,
						self.breathing_color.into_iter().collect(),
					),
				},
				tail: lighting::Tail {
					brightness: self
						.tail_brightness
						.map(RangedByte)
						.unwrap_or(config.lighting.tail.brightness),
					speed: self
						.tail_speed
						.map(RangedByte)
						.unwrap_or(config.lighting.tail.speed),
				},
				fade: lighting::Fade {
					speed: self
						.fade_speed
						.map(RangedByte)
						.unwrap_or(config.lighting.fade.speed),
				},
				rave: lighting::Rave {
					brightness: self
						.rave_brightness
						.map(RangedByte)
						.unwrap_or(config.lighting.rave.brightness),
					speed: self
						.rave_speed
						.map(RangedByte)
						.unwrap_or(config.lighting.rave.speed),
					colors: merge_map(
						config.lighting.rave.colors,
						self.rave_color.into_iter().collect(),
					),
				},
				wave: lighting::Wave {
					brightness: self
						.wave_brightness
						.map(RangedByte)
						.unwrap_or(config.lighting.wave.brightness),
					speed: self
						.wave_speed
						.map(RangedByte)
						.unwrap_or(config.lighting.wave.speed),
				},
				breathing_single: lighting::BreathingSingle {
					speed: self
						.breathing_single_speed
						.map(RangedByte)
						.unwrap_or(config.lighting.breathing_single.speed),
					color: self
						.breathing_single_color
						.unwrap_or(config.lighting.breathing_single.color),
				},
			},
			dpi: {
				let dpi_enable_overrides = {
					let toggle_map = self.toggle_dpi.into_iter().collect::<HashSet<u8>>();
					let enable_map = self.enable_dpi.into_iter().collect::<HashSet<u8>>();
					let disable_map = self.disable_dpi.into_iter().collect::<HashSet<u8>>();

					(0..=5)
						.map(|i| {
							(
								i,
								if toggle_map.contains(&i) {
									Some(!config.dpi[i as usize].enable)
								} else if enable_map.contains(&i) {
									Some(true)
								} else if disable_map.contains(&i) || self.reset_dpis {
									Some(false)
								} else {
									None
								},
							)
						})
						.filter_map(|(i, state)| state.map(|state| (i, state)))
						.collect::<HashMap<u8, bool>>()
				};

				let mut dpi_color_overrides =
					self.dpi_color.into_iter().collect::<HashMap<u8, Color>>();
				let base_dpi_overrides = self.dpi.into_iter().collect::<HashMap<u8, u8>>();
				let x_dpi_overrides = self.dpi_x.into_iter().collect::<HashMap<u8, u8>>();
				let y_dpi_overrides = self.dpi_y.into_iter().collect::<HashMap<u8, u8>>();

				let dpi_overrides = (0..=5)
					.map(|i| {
						(i, Dpi {
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
						})
					})
					.collect::<HashMap<u8, Dpi>>();

				merge_map(config.dpi, dpi_overrides)
			},
			current_dpi: self
				.select_dpi
				.map(RangedByte)
				.unwrap_or(config.current_dpi),
			polling_rate: self.polling_rate.unwrap_or(config.polling_rate),
			liftoff_distance: self.liftoff_distance.unwrap_or(config.liftoff_distance),
			debounce_time: self.debounce_time.unwrap_or(config.debounce_time),
			buttons: MouseButtons {
				left: self.left_button.unwrap_or(config.buttons.left),
				right: self.right_button.unwrap_or(config.buttons.right),
				middle: self.middle_button.unwrap_or(config.buttons.middle),
				forward: self.forward_button.unwrap_or(config.buttons.forward),
				back: self.back_button.unwrap_or(config.buttons.back),
				dpi: self.dpi_button.unwrap_or(config.buttons.dpi),
			},
		}
	}
}
