#[inject(Engine, Manufacturer)]
pub trait Mod {
    // ..
}

pub trait Engine {}
pub trait Manufacturer {}