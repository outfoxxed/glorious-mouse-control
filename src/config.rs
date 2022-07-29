use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct FormatError(pub String);

impl fmt::Display for FormatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "format error: {}", self.0)
	}
}

impl Error for FormatError {}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct ColorSerializer(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "ColorSerializer")]
#[serde(into = "ColorSerializer")]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl TryFrom<&str> for Color {
	type Error = String;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		if value.len() != 6 {
			return Err(format!("could not parse `{}` as hex color", &value));
		}

		(|| {
			Ok(Self {
				r: u8::from_str_radix(&value[0..2], 16)?,
				g: u8::from_str_radix(&value[2..4], 16)?,
				b: u8::from_str_radix(&value[4..6], 16)?,
			})
		})()
		.map_err(|_: ParseIntError| format!("could not parse `{}` as hex color", &value))
	}
}

impl TryFrom<ColorSerializer> for Color {
	type Error = FormatError;

	fn try_from(value: ColorSerializer) -> Result<Self, Self::Error> {
		TryFrom::try_from(&value.0 as &str).map_err(FormatError)
	}
}

impl From<Color> for ColorSerializer {
	fn from(color: Color) -> Self {
		ColorSerializer(format!("{:2x}{:2x}{:2x}", color.r, color.g, color.b))
	}
}

pub mod lighting {
	use super::{Color, FormatError};
	use serde::{Deserialize, Serialize};
	use std::ops::Deref;

	#[derive(Serialize, Deserialize)]
	#[serde(transparent)]
	struct RangedByteSerializer(u8);

	#[derive(Serialize, Deserialize, Debug, Clone)]
	#[serde(try_from = "RangedByteSerializer")]
	#[serde(into = "RangedByteSerializer")]
	pub struct RangedByte<const MIN: u8, const MAX: u8>(pub u8);

	impl<const MIN: u8, const MAX: u8> Deref for RangedByte<MIN, MAX> {
		type Target = u8;

		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}

	impl<const MIN: u8, const MAX: u8> TryFrom<RangedByteSerializer> for RangedByte<MIN, MAX> {
		type Error = FormatError;

		fn try_from(value: RangedByteSerializer) -> Result<Self, Self::Error> {
			if (MIN..=MAX).contains(&value.0) {
				Ok(Self(value.0))
			} else {
				Err(FormatError(format!("{} was not in range {MIN}..{MAX}", value.0)))
			}
		}
	}

	impl<const MIN: u8, const MAX: u8> From<RangedByte<MIN, MAX>> for RangedByteSerializer {
		fn from(byte: RangedByte<MIN, MAX>) -> Self {
			RangedByteSerializer(*byte)
		}
	}

	#[derive(Serialize, Deserialize, Debug, Clone, clap::ArgEnum)]
	pub enum Mode {
		Off,
		Rainbow,
		Solid,
		Breathing,
		Tail,
		Fade,
		WaveSolid,
		Rave,
		Random,
		Wave,
		BreathingSingle,
	}

	#[derive(Serialize, Deserialize, Debug, Clone, clap::ArgEnum)]
	pub enum RainbowDirection {
		Backward,
		Forward,
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Rainbow {
		pub speed: RangedByte<1, 3>,
		pub direction: RainbowDirection,
	}

	impl Default for Rainbow {
		fn default() -> Self {
			Self {
				speed: RangedByte(2),
				direction: RainbowDirection::Backward,
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Solid {
		pub brightness: RangedByte<1, 4>,
		pub color: Color,
	}

	impl Default for Solid {
		fn default() -> Self {
			Self {
				color: Color {
					r: 0xff,
					g: 0xff,
					b: 0xff,
				},
				brightness: RangedByte(4),
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Breathing {
		pub brightness: RangedByte<1, 4>,
		pub speed: RangedByte<1, 3>,
		pub colors: [Color; 7],
	}

	impl Default for Breathing {
		fn default() -> Self {
			Self {
				brightness: RangedByte(4),
				speed: RangedByte(2),
				colors: [
					Color {
						r: 255,
						g: 255,
						b: 255,
					},
					Color {
						r: 255,
						g: 255,
						b: 255,
					},
					Color {
						r: 255,
						g: 255,
						b: 255,
					},
					Color {
						r: 255,
						g: 255,
						b: 255,
					},
					Color {
						r: 255,
						g: 255,
						b: 255,
					},
					Color {
						r: 255,
						g: 255,
						b: 255,
					},
					Color {
						r: 255,
						g: 255,
						b: 255,
					},
				],
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Tail {
		pub brightness: RangedByte<1, 4>,
		pub speed: RangedByte<1, 3>,
	}

	impl Default for Tail {
		fn default() -> Self {
			Self {
				brightness: RangedByte(4),
				speed: RangedByte(2),
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Fade {
		pub speed: RangedByte<1, 3>,
	}

	impl Default for Fade {
		fn default() -> Self {
			Self {
				speed: RangedByte(2),
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Rave {
		pub brightness: RangedByte<1, 4>,
		pub speed: RangedByte<1, 3>,
	}

	impl Default for Rave {
		fn default() -> Self {
			Self {
				brightness: RangedByte(4),
				speed: RangedByte(2),
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Wave {
		pub brightness: RangedByte<1, 4>,
		pub speed: RangedByte<1, 3>,
	}

	impl Default for Wave {
		fn default() -> Self {
			Self {
				brightness: RangedByte(4),
				speed: RangedByte(2),
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct BreathingSingle {
		pub speed: RangedByte<1, 3>,
		pub color: Color,
	}

	impl Default for BreathingSingle {
		fn default() -> Self {
			Self {
				speed: RangedByte(2),
				color: Color {
					r: 255,
					g: 255,
					b: 255,
				},
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug)]
	pub struct Lighting {
		pub mode: Mode,
		pub solid: Solid,
		pub rainbow: Rainbow,
		pub breathing: Breathing,
		pub tail: Tail,
		pub fade: Fade,
		pub rave: Rave,
		pub wave: Wave,
		pub breathing_single: BreathingSingle,
	}

	impl Default for Lighting {
		fn default() -> Self {
			Self {
				mode: Mode::Off,
				solid: Solid::default(),
				rainbow: Rainbow::default(),
				breathing: Breathing::default(),
				tail: Tail::default(),
				fade: Fade::default(),
				rave: Rave::default(),
				wave: Wave::default(),
				breathing_single: BreathingSingle::default(),
			}
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dpi {
	pub enable: bool,
	// TODO: grab defaults from windows software
	pub color: Color,
	// TODO: check if these needs to have a bounded range w/ dpi cap
	pub x_dpi: u8,
	pub y_dpi: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, clap::ArgEnum)]
pub enum PollingRate {
	#[serde(rename = "125Hz")]
	_125,
	#[serde(rename = "250Hz")]
	_250,
	#[serde(rename = "500Hz")]
	_500,
	#[serde(rename = "1000Hz")]
	_1000,
}

#[derive(Serialize, Deserialize, Debug, Clone, clap::ArgEnum)]
pub enum LiftoffDistance {
	#[serde(rename = "2mm")]
	_2,
	#[serde(rename = "3mm")]
	_3,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
	pub lighting: lighting::Lighting,
	pub dpi: [Dpi; 6],
	pub polling_rate: PollingRate,
	pub liftoff_distance: LiftoffDistance,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			lighting: lighting::Lighting::default(),
			dpi: [
				Dpi {
					enable: true,
					color: Color {
						r: 255,
						g: 255,
						b: 255,
					},
					x_dpi: 4,
					y_dpi: 4,
				},
				Dpi {
					enable: true,
					color: Color {
						r: 255,
						g: 255,
						b: 255,
					},
					x_dpi: 8,
					y_dpi: 8,
				},
				Dpi {
					enable: true,
					color: Color {
						r: 255,
						g: 255,
						b: 255,
					},
					x_dpi: 12,
					y_dpi: 12,
				},
				Dpi {
					enable: false,
					color: Color {
						r: 255,
						g: 255,
						b: 255,
					},
					x_dpi: 14,
					y_dpi: 14,
				},
				Dpi {
					enable: false,
					color: Color {
						r: 255,
						g: 255,
						b: 255,
					},
					x_dpi: 16,
					y_dpi: 16,
				},
				Dpi {
					enable: false,
					color: Color {
						r: 255,
						g: 255,
						b: 255,
					},
					x_dpi: 18,
					y_dpi: 18,
				},
			],
			polling_rate: PollingRate::_1000,
			liftoff_distance: LiftoffDistance::_2,
		}
	}
}
