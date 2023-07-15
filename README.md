# stdinman

Tool to pipe raw data from stdin to discord via a bot.

Input data must be PCM 32-bit floating-point little-endian (aka f32le).

## Motivation

There are several interesting discord bots and such floating around, such as for playing Youtube links, soundcloud etc. However, as an **Arch Linux** user, I wanted something super composable - the ability to play _any raw audio_ into Discord, via a bot.

That's exactly what this program does. It relies on having a bot in the server which is connects to a Voice Channel - this is intentional, as if you just want to play audio via your own account, then there are loads of ways via Virtual Microphones and whatnot.

## Recipes

TODO

## Thanks

Many thanks to Enitoni for [pulseshitter](https://github.com/Enitoni/pulseshitter), which was my inspiration for this project.

Also thanks to the amazing developers of [serenity](https://github.com/serenity-rs/serenity/) & [songbird](https://github.com/serenity-rs/songbird/) , for making working with Discord bots and streaming audio in Rust so easy.
