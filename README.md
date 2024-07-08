# Turing Complete "Binary racer" solver

Automated solution finder and executioner for the "Binary racer" level in Turing Complete.
No command line interface is in place yet, you'll have to edit the code manually.

Get the game on [steam](https://store.steampowered.com/app/1444480/Turing_Complete/)

## Description

This assumes default position of the camera in the level at max zoom,
though that can be changed through code and experimentation.
It also assumes the game is running on your second/middle monitor
aka that there's a monitor to the left of the one the game is running on.
Finally, the program checks for exact color values, creates a black and white image
and scores each digit in the string by how many pixels differ to a stored digit.
Digits to compare against are stored in the assets folder.

If I was a little less lazy, I'd convert the color-filtered image to a 1bit map,
perhaps via shaders, and then simply xor and sum the bits of digits to score them for a performance boost.

## Motivation

AI-based or other OCR programs and libraries are a pain to work with so I wanted to try a shot at my own.
I could afford a lazer-focused approach since I'm dealing with one case specifically with no preconception
of what to include if I were to make it modular since that was never the plan for this.

Tesseract failed me on many occasions trying to read even the simplest characters off an image.
There's so many settings it'd take me longer to learn about them and set them up than to write this
(and trust me I've tried).
Even then, it's really picky about what it wants included - if your monochrome image edges don't have anti-aliasing,
good luck trying to detect even the most common of fonts. If you don't have a safety margin around your number,
you might as well not have the number.

Who knows, maybe I was trying to go too quickly for my own good.
Now I at least know it's not a tool you can just pick up and use immediately.

## Requirements

I only ran this on linux so that's what I'm gonna list:

`apt-get install libxcb1 libxrandr2 libdbus-1-3 libxdo-dev`

- `libxcb1`, `libxrandr2`, `libdbus-1-3` are for the `screenshots` crate

- `libxdo-dev` is for the `enigo`


## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
