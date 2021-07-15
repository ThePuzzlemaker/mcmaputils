#![allow(dead_code)]

use image::Rgba;

/// The table of base colors, as RGBA colors fit into a u32.
pub static BASE_COLORS: &[u32] = &[
    0x00000000, 0x7fb238ff, 0xf7e9a3ff, 0xc7c7c7ff, 0xff0000ff, 0xa0a0ffff, 0xa7a7a7ff, 0x007c00ff,
    0xffffffff, 0xa4a8b8ff, 0x976d4dff, 0x707070ff, 0x4040ffff, 0x8f7748ff, 0xfffcf5ff, 0xd87f33ff,
    0xb24cd8ff, 0x6699d8ff, 0xe5e533ff, 0x7fcc19ff, 0xf27fa5ff, 0x4c4c4cff, 0x999999ff, 0x4c7f99ff,
    0x7f3fb2ff, 0x334cb2ff, 0x664c33ff, 0x667f33ff, 0x993333ff, 0x191919ff, 0xfaee4dff, 0x5cdbd5ff,
    0x4a80ffff, 0x00d93aff, 0x815631ff, 0x700200ff, 0xd1b1a1ff, 0x9f5224ff, 0x95576cff, 0x706c8aff,
    0xba8524ff, 0x677535ff, 0xa04d4eff, 0x392923ff, 0x876b62ff, 0x575c5cff, 0x7a4958ff, 0x4c3e5cff,
    0x4c3223ff, 0x4c522aff, 0x8e3c2eff, 0x251610ff, 0xbd3031ff, 0x943f61ff, 0x5c191dff, 0x167e86ff,
    0x3a8e8cff, 0x562c3eff, 0x14b485ff, 0x646464ff, 0xd8af93ff, 0x7fa796ff,
];

// /// A lookup table from color index to color name.
// static BASE_COLOR_NAMES: &[&str] = &[
//     "NONE",
//     "GRASS",
//     "SAND",
//     "WOOL",
//     "FIRE",
//     "ICE",
//     "METAL",
//     "PLANT",
//     "SNOW",
//     "CLAY",
//     "DIRT",
//     "STONE",
//     "WATER",
//     "WOOD",
//     "QUARTZ",
//     "COLOR_ORANGE",
//     "COLOR_MAGENTA",
//     "COLOR_LIGHT_BLUE",
//     "COLOR_YELLOW",
//     "COLOR_LIGHT_GREEN",
//     "COLOR_PINK",
//     "COLOR_GRAY",
//     "COLOR_LIGHT_GRAY",
//     "COLOR_CYAN",
//     "COLOR_PURPLE",
//     "COLOR_BLUE",
//     "COLOR_BROWN",
//     "COLOR_GREEN",
//     "COLOR_RED",
//     "COLOR_BLACK",
//     "GOLD",
//     "DIAMOND",
//     "LAPIS",
//     "EMERALD",
//     "PODZOL",
//     "NETHER",
//     "TERRACOTTA_WHITE",
//     "TERRACOTTA_ORANGE",
//     "TERRACOTTA_MAGENTA",
//     "TERRACOTTA_LIGHT_BLUE",
//     "TERRACOTTA_YELLOW",
//     "TERRACOTTA_LIGHT_GREEN",
//     "TERRACOTTA_PINK",
//     "TERRACOTTA_GRAY",
//     "TERRACOTTA_LIGHT_GRAY",
//     "TERRACOTTA_CYAN",
//     "TERRACOTTA_PURPLE",
//     "TERRACOTTA_BLUE",
//     "TERRACOTTA_BROWN",
//     "TERRACOTTA_GREEN",
//     "TERRACOTTA_RED",
//     "TERRACOTTA_BLACK",
//     "CRIMSON_NYLIUM",
//     "CRIMSON_STEM",
//     "CRIMSON_HYPHAE",
//     "WARPED_NYLIUM",
//     "WARPED_STEM",
//     "WARPED_HYPHAE",
//     "WARPED_WART_BLOCK",
//     "DEEPSLATE",
//     "RAW_IRON",
//     "GLOW_LICHEN",
// ];

/// Convert a full color index (base color index * 4 + shade) to an RGBA color.
pub fn to_rgba_full(idx: u8) -> Rgba<u8> {
    to_rgba_base(idx / 4, Some(idx % 4))
}

/// Convert a base color index along with a shade (optionally) to an RGBA color.
pub fn to_rgba_base(idx: u8, shade: Option<u8>) -> Rgba<u8> {
    let shade = shade.unwrap_or(2);
    let base_color = BASE_COLORS[idx as usize];
    let [r, g, b, a] = base_color.to_be_bytes();

    let mult = match shade {
        0 => 180,
        1 => 220,
        2 => 255,
        3 => 135,
        _ => panic!("invalid shade for map color: `{}`", shade),
    };

    let (r, g, b) = (r as u32, g as u32, b as u32);

    let r = r * mult / 255;
    let g = g * mult / 255;
    let b = b * mult / 255;

    let (r, g, b) = (r as u8, g as u8, b as u8);

    Rgba([r, g, b, a])
}
