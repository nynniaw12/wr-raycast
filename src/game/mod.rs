use std::future::Future;

pub mod demo;
pub mod raycasting;

pub trait Game {
    fn start() -> impl Future<Output = ()>;
}
