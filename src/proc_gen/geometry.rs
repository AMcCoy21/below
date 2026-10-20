use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Edge {
    pub a: usize,
    pub b: usize,
}