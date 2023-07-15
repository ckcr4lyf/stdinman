# stdinman

Tool to pipe raw data from stdin to discord via a bot.

Input data must be PCM 32-bit floating-point little-endian (aka f32le).

## Motivation

There are several interesting discord bots and such floating around, such as for playing Youtube links, soundcloud etc. However, as an **Arch Linux** user, I wanted something super composable - the ability to play _any raw audio_ into Discord, via a bot.

That's exactly what this program does. It is entirely up to the user how they wish to prepare the audio source - for instance their microphone, their speaker output (i.e. alsa monitor), an internet audio stream (or anything outputtable by ffmpeg really), or spinning vinyl*. Check out the [recipes](#recipes) section for more.

## Recipes

TODO

## Thanks

Many thanks to Enitoni for [pulseshitter](https://github.com/Enitoni/pulseshitter), which was my inspiration for this project.

Also thanks to the amazing developers of [serenity](https://github.com/serenity-rs/serenity/) & [songbird](https://github.com/serenity-rs/songbird/) , for making working with Discord bots and streaming audio in Rust so easy.
