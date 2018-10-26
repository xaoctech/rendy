// TODO: module docs

#[macro_use]
pub extern crate ash;
pub extern crate rendy_command as command;
pub extern crate rendy_memory as memory;
pub extern crate rendy_resource as resource;

extern crate winit;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate log;

extern crate relevant;

#[cfg(features = "serde")]
extern crate serde;

extern crate smallvec;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
extern crate cocoa;

mod config;
mod factory;
mod queue;
mod renderer;
mod target;


pub use config::{Config, QueuesConfigure, HeapsConfigure, OneGraphicsQueue, SavedQueueConfig, BasicHeapsConfigure, SavedHeapsConfig};
pub use factory::Factory;
pub use renderer::{Renderer, RendererBuilder};
