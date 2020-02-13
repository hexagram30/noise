# noise

*Noise-generation for use in hexagram30 projects*

[![Project Logo][logo]][logo-large]

Based on the [noise-rs][noise-rs] project.

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

Here's a thumnail of six different cave systems generated with this tool (click
to see large image):

[![Example Outputs][example]][example-large]

<!-- Named page links below: /-->

[logo]: https://raw.githubusercontent.com/hexagram30/resources/master/branding/logo/h30-logo-2-long-with-text-x695.png
[logo-large]: https://raw.githubusercontent.com/hexagram30/resources/master/branding/logo/h30-logo-2-long-with-text-x3440.png
[example]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/caves-small.png
[example-large]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/caves.png
[noise-rs]: https://github.com/Razaekel/noise-rs
