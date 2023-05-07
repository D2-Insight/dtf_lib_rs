use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;

type BungieHash = u32;
type Color = (u8, u8, u8);
type Image = Vec<Vec<Color>>;

#[derive(Serialize, Deserialize)]
struct Response<T> {
    weapon: Weapon,
    buffs: Vec<Buff>,
    settings: Settings,
    results: T,
    submitter: u64,
}

#[derive(Serialize, Deserialize)]
struct Settings {
    brightness: u8,
    fov: u8,
    resolution: (u16, u16),
}

#[derive(Validate, Serialize, Deserialize)]
struct StatValue(
    #[validate(maximum = 100)]
    #[validate(minimum = 0)]
    i32,
);

#[derive(Serialize, Deserialize)]
struct Buff {
    hash: BungieHash,
    value: u8,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Weapon {
    type_hash: u8,
    frame_hash: BungieHash,
    hash: BungieHash,
    stats: HashMap<BungieHash, StatValue>,
}

#[derive(Serialize, Deserialize)]
struct Handling {
    ads_time: f64,
    draw_time: f64,
    stow_time: f64,
}

#[derive(Serialize, Deserialize)]
struct Ammo {
    mag_size: u8,
    inventory_size: u8,
}

#[derive(Serialize, Deserialize)]
struct Reload {
    time: f64,
    ammo_time: f64,
}

#[derive(Serialize, Deserialize)]
struct Reticle(Image);

#[derive(Serialize, Deserialize)]
struct GreenReticle(Vec<Vec<u8>>);