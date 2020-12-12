# interp-toy

This repo contains a Druid-based GUI application for experimenting with sparse interpolation.

As a subdirectory, it contains a copy of the glyphstool crate, for which the authoritative master lives in the [Inconsolata] repository. It's used for reading masters in Glyphs format. If development continues, that should probably be published as a crate.

To try it, run this command:

```
cargo run --release -- glyph testfont-2masters.glyphs n
```

This repo is currently a rough prototype, and uses an old version of [Druid]. It's likely the ideas will be pursued in a different context, for example adding interpolation features to [Runebender].

## License

Licensed under either of
  * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or
    http://opensource.org/licenses/MIT) at your option.

[Inconsolata]: https://github.com/googlefonts/Inconsolata
[Druid]: https://github.com/linebender/druid
[Runebender]: https://runebender.app
