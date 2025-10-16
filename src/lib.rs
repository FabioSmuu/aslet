mod api;
mod backup;
mod error;
mod result;
mod tasks;
mod types;
mod worker;

use godot::prelude::*;

struct AsletExt;

#[gdextension]
unsafe impl ExtensionLibrary for AsletExt {}
