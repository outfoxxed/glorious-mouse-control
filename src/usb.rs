use std::{
	io::{self, Write},
	ops::Deref,
	time::Duration,
};

use rusb::{Device, DeviceHandle};

use crate::{
	config::{self, Color, Dpi, MouseButtonType, RangedByte},
	error,
};

/// List of USB devices to look for `(<vendor id>, <product id>)`.
static TARGET_DEVICES: &[(u16, u16)] = &[
	(0x258a, 0x0033), // Model D
	(0x258a, 0x0036), // Model O
];

/// Finds a device matching one of [`TARGET_DEVICES`].
///
/// # Panics
///
/// If no target device is connected, or there is an error finding it,
/// the program will exit with an error message.
fn find_device() -> Device<rusb::GlobalContext> {
	rusb::devices()
		.unwrap_or_else(|e| error!("could not load usb device list: {e}"))
		.iter()
		.find(|device| {
			let descriptor = device
				.device_descriptor()
				.unwrap_or_else(|e| error!("could not get device descriptor: {e}"));
			let id = (descriptor.vendor_id(), descriptor.product_id());
			TARGET_DEVICES.iter().any(|target| &id == target)
		})
		.unwrap_or_else(|| error!("could not find usb device"))
}

/// Builds a packet matching the `Type 1` section of the `Main Packet`
/// section of `packet_spec.md`. This packet controls DPI values, DPI states,
/// DPI colors, selected DPI, liftoff distance, and RGB.
///
/// # Panics
///
/// If there is an error writing to the command array (probably won't happen),
/// or the selected DPI is not enabled (should be checked before `config` is
/// passed here), the program will exit with an error message.
fn build_main_packet(config: &config::Config) -> [u8; 520] {
	let mut data = io::Cursor::new([0u8; 520]);
	macro_rules! write {
		($buf:expr, [$($data:tt)*]) => {
			$buf.write_all(&[$($data)*]).unwrap_or_else(|e| error!("error writing usb command to buffer: {e}"))
		};
		[$($data:tt)*] => {
			write!(data, [$($data)*])
		};
	}

	let write_color =
		|data: &mut io::Cursor<[u8; 520]>, &Color { r, g, b }| write!(data, [r, b, g]);
	let combine_brightness_speed =
		|brightness: &RangedByte<1, 4>, speed: &RangedByte<1, 3>| (**brightness << 4) | **speed;

	// unknown data
	#[rustfmt::skip]
	write![0x04, 0x11, 0x00, 0x7b, 0x00, 0x00, 0x00, 0x00, 0x64, 0x06];

	let separate_xy_dpi = config
		.dpi
		.iter()
		.any(|Dpi { x_dpi, y_dpi, .. }| x_dpi != y_dpi);

	write![(if separate_xy_dpi { 0x08 } else { 0x00 }) | (config.polling_rate as u8)];

	// dpi enable flags
	let mut dpi_count = 0u8;
	let mut current_dpi = Option::<u8>::None;
	let mut flags = 0u8;

	for (i, flag) in config
		.dpi
		.iter()
		.enumerate()
		.filter(|(_, Dpi { enable, .. })| *enable)
		.map(|(i, _)| (i, 0b1 << i))
	{
		dpi_count += 1;
		if *config.current_dpi as usize == i {
			// current dpi is index (by 1) in the list of enabled dpis
			current_dpi = Some(dpi_count);
		}
		flags |= flag;
	}
	flags = !flags;

	let current_dpi = match current_dpi {
		Some(current_dpi) => current_dpi,
		None => error!("selected DPI is not enabled"),
	};

	write![(current_dpi << 4) | dpi_count, flags];

	if separate_xy_dpi {
		for Dpi { x_dpi, y_dpi, .. } in &config.dpi {
			write![*x_dpi, *y_dpi];
		}
	} else {
		for Dpi { x_dpi: dpi, .. } in &config.dpi {
			write![*dpi];
		}
		write![0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
	}

	// unknown data
	write![0x00, 0x00, 0x00, 0x00];

	// DPI colors
	for Dpi {
		color: Color { r, g, b },
		..
	} in &config.dpi
	{
		write![*r, *g, *b];
	}

	#[rustfmt::skip]
	write![
		// unknown data
		0x00, 0x00, 0x00, 0x00, 0x00,  0x00,
		
		// LED mode
		config.lighting.mode as u8,
		
		// Rainbow speed
		0x40 | *config.lighting.rainbow.speed,
		// Rainbow direction
		config.lighting.rainbow.direction as u8,
		
		// Solid brightness
		*config.lighting.solid.brightness << 4,
	];

	// Solid color
	write_color(&mut data, &config.lighting.solid.color);

	// unknown data
	write![0x42, 0x07];

	// Breathing colors
	for color in &config.lighting.breathing.colors {
		write_color(&mut data, color);
	}

	#[rustfmt::skip]
	write![
		// Tail brightness / speed
		combine_brightness_speed(
			&config.lighting.tail.brightness, 
			&config.lighting.tail.speed
		),
		
		// Fade speed
		0x40 | *config.lighting.fade.speed,
		
		// unknown data (probably tail/fade colors or some combination thereof)
		                                                       0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00,
		0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0xff,  0xff, 0xff, 0xff, 0xff, 0xfa, 0x00, 0xff, 0xff,
		0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0x00,
		
		// Rave brightness / speed
		combine_brightness_speed(
			&config.lighting.rave.brightness,
			&config.lighting.rave.speed,
		),
	];

	for color in &config.lighting.rave.colors {
		write_color(&mut data, color);
	}

	#[rustfmt::skip]
	write![
		// unknown data
		0x02,
		
		// Wave brightness / speed
		combine_brightness_speed(
			&config.lighting.wave.brightness,
			&config.lighting.wave.speed,
		),
		
		// Breathing (Single) speed
		*config.lighting.breathing_single.speed,
	];

	// Breathing (Single) color
	write_color(&mut data, &config.lighting.breathing_single.color);

	// Liftoff distance
	write![config.liftoff_distance as u8];

	data.into_inner()
}

/// Builds a packet matching the `Type 2` section of the `Main Packet` section
/// of `packet_spec.md`. This packet controls mouse button actions.
///
/// # Panics
///
/// If there is an error writing to the command array (probably won't happen)
fn build_buttons_packet(config: &config::Config) -> [u8; 520] {
	let mut data = io::Cursor::new([0u8; 520]);
	macro_rules! write {
		($buf:expr, [$($data:tt)*]) => {
			$buf.write_all(&[$($data)*]).unwrap_or_else(|e| error!("error writing usb command to buffer: {e}"))
		};
		[$($data:tt)*] => {
			write!(data, [$($data)*])
		};
	}

	let write_button = |data: &mut io::Cursor<[u8; 520]>, button: &config::MouseButtonType| {
		data.write_all(&u32::to_be_bytes(*button as u32))
			.unwrap_or_else(|e| error!("error writing usb command to buffer: {e}"));
	};

	// unknown data
	write![0x04, 0x12, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00];

	write_button(&mut data, &config.buttons.left);
	write_button(&mut data, &config.buttons.right);
	write_button(&mut data, &config.buttons.middle);
	write_button(&mut data, &config.buttons.back);
	write_button(&mut data, &config.buttons.forward);
	write_button(&mut data, &config.buttons.dpi);

	for _ in 0..13 {
		write_button(&mut data, &MouseButtonType::Disable);
	}

	data.into_inner()
}

/// Builds a packet matching the `Debounce Packet` section of `packet_spec.md`.
/// This packet controls debounce time.
fn build_debounce_packet(config: &config::Config) -> [u8; 6] {
	[
		0x05, 0x1a, // unknown data
		config.debounce_time as u8,
		0x00, 0x00, 0x00,
	]
}

/// Manages claiming and release of usb device interfaces. (claimed
/// interfaces will be released and reattached to the kernel (if applicable)
/// once this struct is dropped)
struct InterfaceScopeWrapper<'h, const N: usize> {
	interfaces: [(u8, bool); N],
	handle: &'h mut DeviceHandle<rusb::GlobalContext>,
}

impl<'h, const N: usize> InterfaceScopeWrapper<'h, N> {
	fn wrap(handle: &'h mut DeviceHandle<rusb::GlobalContext>, interfaces: [u8; N]) -> Self {
		Self {
			interfaces: interfaces.map(|interface| {
				let attached = rusb::supports_detach_kernel_driver()
					&& handle.kernel_driver_active(interface).unwrap();
				if attached {
					handle.detach_kernel_driver(interface).unwrap();
				}
				handle.claim_interface(interface).unwrap();

				(interface, attached)
			}),
			handle,
		}
	}
}

impl<'h, const N: usize> Drop for InterfaceScopeWrapper<'h, N> {
	fn drop(&mut self) {
		for (interface, attached) in self.interfaces {
			self.handle.release_interface(interface).unwrap();
			if attached {
				self.handle.attach_kernel_driver(interface).unwrap();
			}
		}
	}
}

impl<'h, const N: usize> Deref for InterfaceScopeWrapper<'h, N> {
	type Target = DeviceHandle<rusb::GlobalContext>;

	fn deref(&self) -> &Self::Target {
		self.handle
	}
}

/// Applies the specified `config` to a connected target device.
///
/// # Panics
///
/// See [`build_main_packet`]
pub fn apply_config(config: &config::Config) {
	let device = find_device();

	let main_packet = build_main_packet(config);
	let buttons_packet = build_buttons_packet(config);
	let debounce_packet = build_debounce_packet(config);

	let mut handle = device
		.open()
		.unwrap_or_else(|e| error!("could not open usb device: {e}"));

	let handle = InterfaceScopeWrapper::wrap(&mut handle, [0, 1]);

	handle
		.write_control(0x21, 0x09, 0x0304, 0x1, &main_packet, Duration::from_secs(5))
		.unwrap();
	handle
		.write_control(0x21, 0x09, 0x0304, 0x1, &buttons_packet, Duration::from_secs(5))
		.unwrap();
	handle
		.write_control(0x21, 0x09, 0x0305, 0x1, &debounce_packet, Duration::from_secs(5))
		.unwrap();
}
