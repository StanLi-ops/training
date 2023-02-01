use specs::{Component, NullStorage, VecStorage, World, WorldExt};
use std::fmt;
use std::fmt::Display;

#[derive(PartialEq)]
pub enum BoxColour {
    Red,
    Blue,
}
impl Display for BoxColour {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
        })?;
        Ok(())
    }
}

//渲染组件状态
pub enum RenderableKind {
    Static,
    Animated,
}

//位置组件（存放位置参数）
#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

//渲染组件（存放渲染所需的图片路径）
#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    paths: Vec<String>,
}
impl Renderable {
    pub fn new_static(path: String) -> Self {
        Self { paths: vec![path] }
    }
    pub fn new_animated(paths: Vec<String>) -> Self {
        Self { paths }
    }
    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }
    pub fn path(&self, path_index: usize) -> String {
        // If we get asked for a path that is larger than the
        // number of paths we actually have, we simply mod the index
        // with the length to get an index that is in range.
        self.paths[path_index % self.paths.len()].clone()
    }
}

//墙-实体
#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

//玩家-实体
#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

//箱子-实体
#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
    pub colour: BoxColour,
}

//箱子点-实体
#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub colour: BoxColour,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immaovable;

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Immaovable>();
    world.register::<Movable>();
}
