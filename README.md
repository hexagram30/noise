# noise

*Noise-generation for use in hexagram30 projects*

[![Project Logo][logo]][logo-large]

Based on the [noise-rs][noise-rs] project.

## Examples

To see the (small) examples, run `make examples`. Images will be generated in
`./example_images`.

## Building

Run `make`.

## Usage

After building, run `./bin/noise help`, `./bin/noise -h`, and `./bin/noise cave -h`.

The following will generate a `test.png` image in the `./example_images` directory:

```
./bin/noise -s 2 -r 800,800 -t -0.2 -o test.png cave complex
```

<!-- Named page links below: /-->

[logo]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/caves-small.png
[logo-large]: https://raw.githubusercontent.com/hexagram30/noise/master/assets/images/caves.png
[noise-rs]: https://github.com/Razaekel/noise-rs
