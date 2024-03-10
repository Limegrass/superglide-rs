# Superglide-rs

A Rust implementation of the [PowerShell script][powershell-script].
As the script above was implemented in powershell, using it on Linux was not particularly smooth.
While Rust doesn't come on every machine, the binary created should be portable.

As this is closer to your PC's metal than browsers,
it's fairly likely that this is more accurate than the [superglide trainer website][superglide-web],
which notes that most browsers are limited in their polling rates.
However, your mileage may vary.

## Installation

Download the binary and call it with your desired frame-rate, or compile from source.
See https://rustup.rs for help installing the Rust toolchain.
Options should be available within the help text.

[powershell-script]: https://github.com/AngryGroceries/Apex_Superglide_Practice_Tool
[superglide-web]: https://apexmovement.tech/superglidetrainer/
