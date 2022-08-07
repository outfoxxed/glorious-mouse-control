# glorious-mouse-control

CLI control for Glorious Model D and Model O (wired) mice.

Currently provides control for
 - [X] DPI Presets
 - [X] DPI Colors
 - [X] Selected DPI
 - [X] Liftoff distance
 - [ ] Debounce Time
 - [X] RGB
 - [X] Button Mapping

Since both targeted mice send all of their settings in only a few large packets,
configuration state is saved in `~/.config/glorious-mouse-control/config.json` (dependent on your XDG config directory).

All commands that change settings also update the current config. Running with no arguments will apply the current configuration.

### Warning: Running this program *will* reset your mouse configuration*
*If it was not previously set by this program it will be reset.
If it was previously set by this program, the config file will be used.

### Usage

```
glorious-mouse-control 

USAGE:
    glorious-mouse-control [OPTIONS]

OPTIONS:
        --back-button <BACK_BUTTON>
            Left mouse button action [possible values: disable, left-click, right-click,
            middle-click, back, forward, scroll-up, scroll-down, dpi-loop, dpi-plus, dpi-minus]

        --breathing-brightness <BREATHING_BRIGHTNESS>
            LED brightness in Breathing mode (1-4)

        --breathing-color <BREATHING_COLOR>
            Set Breathing color (<index 0-6>:<hex color>)

        --breathing-single-color <BREATHING_SINGLE_COLOR>
            LED color in Solid mode (hex)

        --breathing-single-speed <BREATHING_SINGLE_SPEED>
            LED animation speed in Breathing (Single) mode (1-3)

        --breathing-speed <BREATHING_SPEED>
            LED animation speed in Breathing mode (1-3)

        --disable-dpi <DISABLE_DPI>
            Disable a DPI setting (0-5)

        --dpi <DPI>
            Set the X and Y DPI for a DPI setting (<index 0-5>:<dpi ending in 00>)

        --dpi-button <DPI_BUTTON>
            Left mouse button action [possible values: disable, left-click, right-click,
            middle-click, back, forward, scroll-up, scroll-down, dpi-loop, dpi-plus, dpi-minus]

        --dpi-color <DPI_COLOR>
            Set the color for a DPI setting (<index 0-5>:<hex color>))

        --dpi-x <DPI_X>
            Set the X DPI for a DPI setting (<index 0-5>:<dpi ending in 00>)

        --dpi-y <DPI_Y>
            Set the Y DPI for a DPI setting (<index 0-5>:<dpi ending in 00>)

        --enable-dpi <ENABLE_DPI>
            Enable a DPI setting (0-5)

        --fade-speed <FADE_SPEED>
            LED animation speed in Fade mode (1-3)

        --forward-button <FORWARD_BUTTON>
            Left mouse button action [possible values: disable, left-click, right-click,
            middle-click, back, forward, scroll-up, scroll-down, dpi-loop, dpi-plus, dpi-minus]

    -h, --help
            Print help information

        --left-button <LEFT_BUTTON>
            Left mouse button action [possible values: disable, left-click, right-click,
            middle-click, back, forward, scroll-up, scroll-down, dpi-loop, dpi-plus, dpi-minus]

        --liftoff-distance <LIFTOFF_DISTANCE>
            Set liftoff distance (millimeters) [possible values: 2, 3]

        --middle-button <MIDDLE_BUTTON>
            Left mouse button action [possible values: disable, left-click, right-click,
            middle-click, back, forward, scroll-up, scroll-down, dpi-loop, dpi-plus, dpi-minus]

        --mode <MODE>
            LED lighting mode [possible values: off, rainbow, solid, breathing, tail, fade,
            wave-solid, rave, random, wave, breathing-single]

        --polling-rate <POLLING_RATE>
            Set polling rate [possible values: 125, 250, 500, 1000]

        --rainbow-direction <RAINBOW_DIRECTION>
            LED animation direction in Rainbow mode [possible values: backward, forward]

        --rainbow-speed <RAINBOW_SPEED>
            LED animation speed in Rainbow mode (1-3)

        --rave-brightness <RAVE_BRIGHTNESS>
            LED brightness in Rave mode (1-4)

        --rave-color <RAVE_COLOR>
            Set Rave color (<index 0-1>:<hex color>)

        --rave-speed <RAVE_SPEED>
            LED animation speed in Rave mode (1-3)

        --reset-dpis
            Reset dpis not listed

        --right-button <RIGHT_BUTTON>
            Left mouse button action [possible values: disable, left-click, right-click,
            middle-click, back, forward, scroll-up, scroll-down, dpi-loop, dpi-plus, dpi-minus]

        --select-dpi <SELECT_DPI>
            Set current DPI

        --solid-brightness <SOLID_BRIGHTNESS>
            LED brightness in Solid mode (1-4)

        --solid-color <SOLID_COLOR>
            LED color in Solid mode (hex)

        --tail-brightness <TAIL_BRIGHTNESS>
            LED brightness in Tail mode (1-4)

        --tail-speed <TAIL_SPEED>
            LED animation speed in Tail mode (1-3)

        --toggle-dpi <TOGGLE_DPI>
            Toggle a DPI setting (0-5)

        --wave-brightness <WAVE_BRIGHTNESS>
            LED brightness in Wave mode (1-4)

        --wave-speed <WAVE_SPEED>
            LED animation speed in Wave mode (1-3)

```

### Common errors

```
error: could not open device: Access denied (insufficient permissions)
```
You may need to run the program as root, or grant write access to your mouse some other way.

### Examples

Apply the current configuration:
```
glorious-mouse-control // no arguments
```

Set one DPI and select it
```
glorious-mouse-control --reset-dpis --enable-dpi 0 --dpi 0:1100 --dpi-color 0:ffffff --select-dpi 0
```

Enable RGB in Breathing mode with custom colors
```
glorious-mouse-control --mode breathing --breathing-color 0:ff0000 --breathing-color 1:00ff00 --breathing-color 2:0000ff --breathing-color 3:ff00ff --breathing-color 4:ffff00 --breathing-color 5:00ffff --breathing-color 6:ffffff --breathing-speed 3
```

Disable RGB
```
glorious-mouse-control --mode off
```

Set the DPI button to middle-click
```
glorious-mouse-control --dpi-button middle-click
```
