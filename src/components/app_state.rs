#![warn(clippy::pedantic)]

use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    InGame,
    GameOver,
}