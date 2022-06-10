#![warn(clippy::pedantic)]

use crate::prelude::*;

#[derive(Component, PartialEq)]
pub enum Collider {
    Solid,
}