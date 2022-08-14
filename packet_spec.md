# USB Packet Specification
This is the packet specification used by the program. It was sniffed from
the official glorious software, and as far as I know, is correct*.

*This specification has had multiple corrections, so its likely theres still
a few things wrong with it.

Bytes marked as unknown have not been identified by changing settings.
Even so, they might change something I'm unaware of.

### Main Packet
 - Request Type: `0x21`
 - Request: `0x09`
 - Value: `0x0304`
 - Index: `0x1`

#### Type 1 (DPI + RGB + Polling Rate + LOD)

`04 11 00 7b 00 00 00 00 64 06` - Unknown

`XY` - XY Independent / Polling Rate
 - `X` - XY Independent
   - `0` - Off
   - `8` - On
 - `Y` - Polling Rate
   - `1` - 125Hz
   - `2` - 250Hz
   - `3` - 500Hz
   - `4` - 1000Hz

`XY` - DPI enable count
 - `X` - Selected DPI (only counts enabled DPIs, from 1)
 - `Y` = Enabled DPI count

DPI Enable State (bitset)
- `11111110` - DPI 1
- `11111101` - DPI 2
- `11111011` - DPI 3
- `11110111` - DPI 4
- `11101111` - DPI 5
- `11011111` - DPI 6

DPI
 - XY Independent off
   - `XX` - DPI 1 (hundreds)
   - `XX` - DPI 2 (hundreds)
   - `XX` - DPI 3 (hundreds)
   - `XX` - DPI 4 (hundreds)
   - `XX` - DPI 5 (hundreds)
   - `XX` - DPI 6 (hundreds)
   - `00 00 00 00 00 00`
 - XY Independent on
   - `XX XX` - DPI 1 (hundreds)
   - `XX XX` - DPI 2 (hundreds)
   - `XX XX` - DPI 3 (hundreds)
   - `XX XX` - DPI 4 (hundreds)
   - `XX XX` - DPI 5 (hundreds)
   - `XX XX` - DPI 6 (hundreds)

`00 00 00 00` - Unknown

`XX XX XX` - DPI Color 1 (RGB)

`XX XX XX` - DPI Color 2 (RGB)

`XX XX XX` - DPI Color 3 (RGB)

`XX XX XX` - DPI Color 4 (RGB)

`XX XX XX` - DPI Color 5 (RGB)

`XX XX XX` - DPI Color 6 (RGB)

`00 00 00 00 00 00` - Unknown

LED Mode
 - `00` - Off
 - `01` - Rainbow / Glorious Mode
 - `02` - Solid
 - `03` - Breathing
 - `04` - Tail
 - `05` - Fade / Seamless Breathing
 - `06` - Wave, but it dosent move
 - `07` - Rave
 - `08` - Random
 - `09` - Wave
 - `0a` - Breathing (Single Color)

`Rainbow` Speed
- (undocumented)
- `36-39` - Really slow
- `40` - Fast
- `41-43` - Exposed in Glorious software
- `46-49` - Broken
- `50-*` - Fast

`Rainbow` Direction
- `00` - Backward
- `01` - Forward

`00-40` - `Solid` Brightness (only uses first hex digit)

`XX XX XX` - `Solid` Color (RBG)

`42 07` - Unknown

`XX XX XX` - `Breathing` Color 1 (RBG)

`XX XX XX` - `Breathing` Color 2 (RBG)

`XX XX XX` - `Breathing` Color 3 (RBG)

`XX XX XX` - `Breathing` Color 4 (RBG)

`XX XX XX` - `Breathing` Color 5 (RBG)

`XX XX XX` - `Breathing` Color 6 (RBG)

`XX XX XX` - `Breathing` Color 7 (RBG)

`XY` - `Tail` Brightness / Speed
 - `X` - `0-4` - Brightness
 - `Y` - `1-3` - Speed

`4X` - `Fade` Speed
 - `4` - Appears to be No-Op
 - `X` - `0-3` Speed (`0` is fast and broken)

```
                           00 ff 00 00 00 ff 00
00 00 ff ff ff 00 00 ff ff ff ff ff fa 00 ff ff
00 00 ff 00 00 ff 00 00 
```
Unknown

`XY` - `Rave` Brightness / Speed
 - `X` - `0-4` - Brightness
 - `Y` - `1-3` - Speed

`XX XX XX` - `Rave` Color 1 (RBG)

`XX XX XX` - `Rave` Color 2 (RBG)

`02` - Unknown

`XY` - `Wave` Speed / Brightness
 - `X` - `0-4` - Brightness
 - `Y` - `1-3` - Speed

`00-03` - `Breathing (Single Color)` Speed (`00` = Solid)

`XX XX XX` - `Breathing (Single Color)` Color (RBG)

Liftoff Distance (LOD)
 - `01` - 2mm
 - `02` - 3mm

390x `00`

#### Type 2 (Mouse buttons)

`04 12 00 50 00 00 00 00` - Unknown

Mouse Button Types
 - `50 01 00 00` - Disable
 - `11 01 00 00` - Left click
 - `11 02 00 00` - Right click
 - `11 04 00 00` - Middle click
 - `11 08 00 00` - Back
 - `11 10 00 00` - Forward
 - `12 01 00 00` - Scroll Up
 - `12 ff 00 00` - Scroll Down
 - `31 01 32 03` - Three click
 - `41 00 00 00` - DPI Loop
 - `41 01 00 00` - DPI +
 - `41 02 00 00` - DPI -
 - `22 00 01 00` - Media Player
 - `22 08 00 00` - Media Play / Pause
 - `22 01 00 00` - Media Next
 - `22 02 00 00` - Media Previous
 - `22 04 00 00` - Media Stop
 - `22 10 00 00` - Media Mute
 - `22 40 00 00` - Volume Up
 - `22 80 00 00` - Volume Down
 - `22 00 10 00` - Email
 - `22 00 20 00` - Calculator
 - `22 00 02 00` - Explorer
 - `22 00 00 02` - Home Page
 - `42 03 00 00` - DPI Lock: 400
 - `42 04 00 00` - DPI Lock: 500
 - `42 05 00 00` - DPI Lock: 600
 - `42 06 00 00` - DPI Lock: 700
 - `42 07 00 00` - DPI Lock: 800
 - `42 08 00 00` - DPI Lock: 900
 - `42 09 00 00` - DPI Lock: 1000
 - `42 0a 00 00` - DPI Lock: 1100
 - `42 0b 00 00` - DPI Lock: 1200
 - `42 0c 00 00` - DPI Lock: 1300

`XX XX XX XX` - Left Mouse Button (One of Mouse Button Types)

`XX XX XX XX` - Right Mouse Button (One of Mouse Button Types)

`XX XX XX XX` - Middle Mouse Button (One of Mouse Button Types)

`XX XX XX XX` - Back Side Mouse Button (One of Mouse Button Types)

`XX XX XX XX` - Forward Side Mouse Button (One of Mouse Button Types)

`XX XX XX XX` - DPI Button (One of Mouse Button Types)

13x `50 01 00 00` - Unused Button - Type: Disabled

432x `00`

### Debounce Packet

`05 1a` - Unknown

Debounce Time
 - `02` - 4ms
 - `03` - 6ms
 - `04` - 8ms
 - `05` - 10ms
 - `06` - 12ms
 - `07` - 14ms
 - `08` - 16ms

3x `00`
