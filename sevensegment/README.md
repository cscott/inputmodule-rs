# Seven Segment Display

It's a 9-digit 7-segment display, controlled by RP2040 MCU and IS31FL3729
LED controller.

Connection to the host system is via USB 2.0 and currently there is a USB Serial API to control it without reflashing.

- Commands
  - Display various percentages
  - Change brightness
  - A commandline script and graphical application to control it
- Sleep Mode
  - Transition slowly turns off/on the LEDs

## Controlling

### Commandline

```
> inputmodule-control seven-segment
LED Matrix

Usage: ipc led-matrix [OPTIONS]

Options:
      --brightness [<BRIGHTNESS>]
          Set LED max brightness percentage or get, if no value provided
      --sleeping [<SLEEPING>]
          Set sleep status or get, if no value provided [possible values: true, false]
      --bootloader
          Jump to the bootloader
      --percentage <PERCENTAGE>
          Display a percentage (0-100)
      --animate [<ANIMATE>]
          Start/stop animation [possible values: true, false]
      --pattern <PATTERN>
          Display a pattern [possible values: percentage, gradient, double-gradient, lotus-sideways, zigzag, all-on, panic, lotus-top-down]
      --all-brightnesses
          Show every brightness, one per segment
      --blinking
          Blink the current pattern once a second
      --breathing
          Breathing brightness of the current pattern
      --clock
          Show the current time
      --string <STRING>
          Display a string (max 5 chars)
      --set-color <SET_COLOR>
           Set the legend backlight color [possible values: white, black, red, green, blue, yellow, cyan, purple]

      --animation-fps [<ANIMATION_FPS>]
          Set/get animation FPS
      --panic
          Crash the firmware (TESTING ONLY!)
  -v, --version
          Get the device version
  -h, --help
          Print help
```

### Non-trivial Examples

Most commandline arguments should be self-explanatory.
If not, please open an issue.
Those that require an argument or setup have examples here:

###### Displaying strings

We can display 9-18 characters on the display (depending on how many
decimal point/periods there are).

```sh
inputmodule-control led-matrix --string "A...B...C..."
```

###### Percentage

Light up a percentage of the module, as a sequence of 8s from bottom to top.
This could be used to show volume level, progress of something, or similar,
but is probably not really useful.

```sh
inputmodule-control led-matrix --percentage 30
```

## Sleep Behavior

Currently sleeping means all LEDs and the LED controller are turned off.
Transitions of sleep state slowly fade the LEDs on or off.

Optionally the firmware can be configured, at build-time, to turn the LEDs
on/off immediately. Or display "SLEEP" instead of turning the LEDs off, which
is useful for debugging whether the device is sleeping or not powered.


###### Changing Sleep State

What can change the sleep state

- Hardware/OS triggers
  - `SLEEP#` pin
  - USB Suspend
- Software/Firmware Triggers
  - Sleep/Wake or other command via USB Serial
  - Idle timer

Both of the hardware/OS triggers change the sleep state if they transition from one state to another.
For example, if USB suspends, the LED matrix turns off. If it resumes, the LEDs come back on.
Same for the `SLEEP#` pin.
If either of them indicates sleep, even if they didn'td change state, the module goes to sleep.
If they're active, they don't influence module state. That way sleep state can be controlled by commands and isn't overridden immediately.

The sleep/wake command always changes the state. But it can't be received when USB is suspended.
Any other command will also wake up the device.

The idle timer will send the device to sleep after a configured timeout (default 60 seconds).
The idle timer is reset once the device wakes up or once it receives a command.

## DIP Switch

LED Matrix hardware has a DIP switch with two
switches, let's call them DIP1 and DIP2.  The next version of the
Seven Segment Display module will probably have them as well.

###### DIP2 (Bootloader)

DIP2 is the bootloader switch. To enter bootloader mode follow these steps:

1. Unplug module and flip the switch to ON
2. Plug module back in, it will appear as a flash drive with the name `RPI-RP2`
3. Copy the firmware `.uf2` file onto that drive, it will automatically flash and reappear as a flash drive
4. To exit bootloader mode, unplug the module to flip the switch back, and plug it back in
5. Now the new firmware should be running

As a side effect of being in bootloader mode, the LEDs all stay off.

###### DIP1 (General Purpose)

DIP1 could serve many purposes. Currently it is configured to enable the debug mode.
When debug mode is enabled and the module goes to sleep, it will not turn the LEDs off to save power.
Instead it will display the reason why it went to sleep. This is useful for debugging module and host system behavior.
Debug mode will start up to a fully lit matrix and never goes to sleep based on a timeout.

Sleep Reasons can be:

- `SLEEP#` pin: `SLEEP pin`
- USB Suspend: `USB`
- Command: `COMMAND`
- Timeout: `T. OUT`
