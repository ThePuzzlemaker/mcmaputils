# Minecraft Map Utilities

Utilties for converting images to and from Minecraft maps.

## Subcommands

- `map2image` (or `map2img`/`m2i`) converts a Minecraft map (in the form of a `map_<#>.dat` file, which is located in the `data` folder of a Minecraft world) into a PNG, JPG, or BMP image, with a customizable integer scaling factor.
  - This currently uses data from 1.17.1 (DataVersion `2730`), so please open an issue if anything is incompatible with an older version. 
  - The [`image`](https://github.com/image-rs/image) crate supports more image types, so if there is one that it supports that you need as a supported format, please open an issue or a PR.
- `image2map` is TODO and will be completed soon(ish).

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.