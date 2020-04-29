# noise

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

*Noise-generation for use in hexagram30 projects*

[![Project Logo][logo]][logo-large]

Based on the [noise-rs][noise-rs] project. Implements two custom noise
modifiers used for cave generation: `threshold` and `invert`.

## Features

Currently supports cave-like noise generation via the CLI `cave` subcommand,
with six different cave types to choose from.

## Examples

To see the (small) examples, run `make examples`. Images will be generated in
the `./example_images` directory.

## Building

Run `make`.

## Usage

After building, run the following for detailed option and flag info:

* `./bin/hxgm30-noise help`
* `./bin/hxgm30-noise -h`
* `./bin/hxgm30-noise cave -h`

The following will generate a `test.png` image in the `./example_images` directory:

```
./bin/hxgm30-noise -s 2 -r 800,800 -t -0.2 -o test.png cave complex
```

## Example Output

Here's a thumnail of six different cave systems generated with this tool (click
to see large image):

[![Example Outputs][example]][example-large]

Here's one of them generated with the ASCII output option:

[![Cave ASCII Output][cavesea-screen]][cave-screen-large]

And here's another one colored for land and sea:

[![Land/Sea ASCII Output][land-sea-screen]][land-sea-screen-large]

Here's one generated with more than two levels (and colors), useful for things like land cover:

[![Land Cover ASCII Output][land-cover-screen]][land-cover-screen-large]


<!-- Named page links below: /-->

[logo]: https://raw.githubusercontent.com/hexagram30/resources/master/branding/logo/h30-logo-2-long-with-text-x695.png
[logo-large]: https://raw.githubusercontent.com/hexagram30/resources/master/branding/logo/h30-logo-2-long-with-text-x3440.png
[cave-screen]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/screenshot-caves-complex-billow-thumb.png
[cave-screen-large]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/screenshot-caves-complex-billow.png
[land-sea-screen]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/screesnhot-land-sea-fractured-hm-thumb.png
[land-sea-screen-large]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/screesnhot-land-sea-fractured-hm.png
[land-cover-screen]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/screenshot-levels-wobbly-walls-turbulence-thumb.png
[land-cover-screen-large]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/screenshot-levels-wobbly-walls-turbulence.png
[example]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/caves-small.png
[example-large]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/caves.png
[noise-rs]: https://github.com/Razaekel/noise-rs
[build]: https://github.com/hexagram30/noise/actions?query=workflow%3Abuild+
[build-badge]: https://github.com/hexagram30/noise/workflows/build/badge.svg
[crate]: https://crates.io/crates/hxgm30-noise
[crate-badge]: https://img.shields.io/crates/v/hxgm30-noise.svg
[docs]: https://docs.rs/hxgm30-noise/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/v/tag/hexagram30/noise.svg?sort=semver
[tag]: https://github.com/hexagram30/noise/tags
