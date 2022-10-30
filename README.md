[![build](https://github.com/haimgel/display-switch/workflows/build/badge.svg?branch=master)](https://github.com/haimgel/display-switch/actions)
[![GitHub license](https://img.shields.io/github/license/haimgel/display-switch)](https://github.com/haimgel/display-switch/blob/master/LICENSE)

# display-switch with spotify

A _really_ scuffed fork of `display-switch` that will automatically transfer spotify playback.

**Don't actually use this fork! Use [daniel5151/music-transfer](https://github.com/daniel5151/music-transfer) instead!**

It works, but just barely. I've already been using it locally for over a month, and i'd wager that it works a solid 95% of the time (i.e: I do need to kick it back on occasionally).

Maybe i'll polish up this fork up a bit at some point, but probably not lol

## New Settings

This fork includes a couple new config file settings:

```ini
# create an app via https://developer.spotify.com/dashboard/applications
spotify_client_id = "..."
spotify_client_secret = "..."
spotify_redirect_url = "..."
# ...and select which devices to transfer playback between
spotify_on_usb_connect = "DEVKEEPER"
spotify_on_usb_disconnect = "DOOMKEEPER"
```

## Spotify setup

Requires running the included `rspotify` binary whenever the spotify client / redirect uri settings change. this will generate a valid oauth2 token, which will be cached in `.spotify_token_cache.json`.

```bash
cargo run -p rspotify
```

Once the cache file is generated, make sure it's in the same directory as the actual display_switch executable.

# Turn a $30 USB switch into a full-featured KVM

This utility watches for USB device connect/disconnect events and switches monitor inputs via DDC/CI. This turns
a simple USB switch into a full-fledged KVM solution: press one button on your USB switch and all your monitors
connect to a different input.

It is supposed to be installed on all computers that could be connected to these monitors, since the app only switches
monitors "one way" and relies on itself running on the other computers to switch it "the other way" as needed.

## Platforms supported

The app should function on MacOS, Windows, and Linux. Most of the code is in Rust, with the exception of DDC support
on MacOS, which is done via statically-linked Swift library.

## Configuration

The configuration is pretty similar on all platforms:

On MacOS: the configuration file is expected in `~/Library/Preferences/display-switch.ini`
On Windows: the configuration file is expected in `%APPDATA%\display-switch\display-switch.ini`
On Linux: the configuration file is expected in `$XDG_CONFIG_HOME/display-switch/display-switch.ini` or `~/.config/display-switch/display-switch.ini`

Configuration file settings:

```ini
  usb_device = "1050:0407"
  on_usb_connect = "Hdmi1"
  on_usb_disconnect = "Hdmi2"
```

`usb_device` is which USB device to watch (vendor id / device id in hex), and `on_usb_connect` is which monitor input
to switch to, when this device is connected. Supported values are `Hdmi1`, `Hdmi2`, `DisplayPort1`, `DisplayPort2`, `Dvi1`.
If your monitor has an USB-C port, it's usually reported as `DisplayPort2`. Input can also be specified as a "raw"
decimal or hexadecimal value: `on_usb_connect = 0x10`

The optional `on_usb_disconnect` settings allows to switch in the other direction when the USB device is disconnected.
Note that the preferred way is to have this app installed on both computers. Switching "away" is problematic: if the
other computer has put the monitors to sleep, they will switch immediately back to the original input.

### Different inputs on different monitors
`display-switch` supports per-monitor configuration: add one or more monitor-specific configuration sections to set
monitor-specific inputs. For example:

```ini
on_usb_connect = "DisplayPort2"
on_usb_disconnect = "Hdmi1"

[monitor1]
monitor_id = "len"
on_usb_connect = "DisplayPort1"

[monitor2]
monitor_id = "dell"
on_usb_connect = "hdmi2"
```

`monitor_id` specifies a case-insensitive substring to match against the monitor ID. For example, 'len' would match
`LEN P27u-10 S/N 1144206897` monitor ID. If more than one section has a match, a first one will be used.
`on_usb_connect` and `on_usb_disconnect`, if defined, take precedence over global defaults.

### USB Device IDs

#### Windows
To locate the ID of your USB device ID on Windows:
1. Open Device Manager
2. Locate the USB device, view the properties
3. Switch to the *Details* tab and select *Hardware IDs* in the Property dropdown
4. You should see a value similar to `HID\VID_046D&PID_C52B&MI_00` (the exact values will differ) - the USB device ID is a combination of the *Vendor ID* and the *Product ID* - for example, in this case it would be `046D:C52B`

#### MacOS
To locate the ID of your USB device ID on MacOS, open a terminal and run the following:
```bash
brew install lsusb

$ lsusb > a
<switch the usb dock here>
$ lsusb > b
$ opendiff a b
```
In the command output, the highlighted lines show you which USB IDs are most relevant.

#### Linux
To locate the ID of your USB device on Linux, first install `lsusb`, which your Linux
distro should have a package for. (On Debian, Ubuntu and RedHat, the package name is `usbutils`.)
Then, in a terminal, run the following:
```
$ lsusb > a
<switch the usb dock here>
$ lsusb > b
$ diff -u a b
```
The diff output will show which USB IDs are most relevant.

## Logging

* On MacOS: the log file is written to `/Users/USERNAME/Library/Logs/display-switch/display-switch.log`
* On Windows: the log file is written to `%LOCALAPPDATA%\display-switch\display-switch.log`
* On Linux: The log file is written to `$XDG_DATA_HOME/display-switch/display-switch.log`
 or `~/.local/share/display-switch/display-switch.log`

## Building from source

### Windows

[Install Rust](https://www.rust-lang.org/tools/install), then do `cargo build --release`

### MacOS

[Install Xcode](https://developer.apple.com/xcode/), [install Rust](https://www.rust-lang.org/tools/install), then do
`cargo build --release`

### Linux

[Install Rust](https://www.rust-lang.org/tools/install), then do `cargo build --release`

## Running on startup

### Windows

Copy `display_switch.exe` from `target\release` (where it was built in the previous step) to
`%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup` (replace Username with your
Windows user name).

### MacOS

```bash
  # Get your INI file in order! (see above)
  cp target/release/display_switch /usr/local/bin
  cp dev.haim.display-switch.daemon.plist ~/Library/LaunchAgents/
  launchctl load ~/Library/LaunchAgents/dev.haim.display-switch.daemon.plist
```
### Linux
Copy built executable:

```bash
  cp target/release/display_switch /usr/local/bin
```
Enable read/write access to i2c devices for users in `i2c` group. Run as root :

```bash
groupadd i2c
echo 'KERNEL=="i2c-[0-9]*", GROUP="i2c"' >> /etc/udev/rules.d/10-local_i2c_group.rules
udevadm control --reload-rules && udevadm trigger
```

Then add your user to the i2c group :

```
sudo usermod -aG i2c $(whoami)
```

Create a systemd unit file in your user directory (`/home/$USER/.config/systemd/user/display-switch.service`) with contents

```
[Unit]
Description=Display switch via USB switch

[Service]
ExecStart=/usr/local/bin/display_switch
Type=simple
StandardOutput=journal
Restart=always

[Install]
WantedBy=default.target
```

Create the config file at `/home/$USER/.config/display-switch/display-switch.ini`.
Then enable the service with

```bash
systemctl --user daemon-reload
systemctl --user enable display-switch.service
systemctl --user start display-switch.service
```
