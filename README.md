# `commute`

Do you use your linux computer for both work and personal use?
Do you get annoyed at seeing all your work tabs at the weekend?
Well look no further, for `commute` is a small program to automatically change the appearance and behaviour of your desktop in line with work hours.

Run just `commute` to automatically choose whether to set a work or a home profile.
Doing so will set your colour scheme, browser and select a random background image.
For the best experience, put this in a cron job.

Run `commute home` to set the home presets and override the automatic choice for the next few hours (or a specified length of time).

Run `commute work` to set the work presets and override as with `commute home`.

Run `commute config` to inspect and change config.

For more information, run `commute help`.

## Installation from source

To compile the source, run
```bash
sudo apt install libglib2.0-dev # on ubuntu, or equivalent for your distro
cargo build --release
sudo install -m755 ./target/release/commute /usr/bin/commute
```
