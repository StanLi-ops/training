use core::time::Duration;
use ggez::event::KeyCode;
use specs::World;
use std::fmt;
use std::fmt::Display;

use crate::audio::AudioStore;
use crate::events::Event;


//资源-用于存储按键的消息队列
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

//资源-用于存储游戏状态
pub enum GameplayState {
    Playing,
    Won,
    // Lose,
}
impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}
impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

//注册资源方法
pub fn register_resoures(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
}
