use godot::prelude::*;

use crate::tasks::TaskContext;

/// Represents an asynchronous operation in progress.
///
/// An `AsletTask` can be canceled before it completes, and will emit
/// the [`done`] signal once finished.
#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct AsletTask {
    ctx: TaskContext,
}

#[godot_api]
impl AsletTask {
    /// Emitted when the task finishes execution.
    /// The `result` is an array where:
    /// - the first element is `OK` or `FAILED`,
    /// - if the first element is `OK`, the second element contains the operation's data.
    #[signal]
    fn done(result: Variant);

    /// Creates a new [`AsletTask`] with the given internal state.
    pub fn new(ctx: TaskContext) -> Gd<Self> {
        Gd::from_object(Self { ctx })
    }

    /// Attempts to cancel the task if it is still waiting.
    ///
    /// Returns:
    /// - `OK` if the task was successfully canceled.
    /// - `FAILED` if the task was already running or finished.
    #[func]
    pub fn cancel(&self) -> godot::global::Error {
        if self.ctx.cancel() {
            godot::global::Error::OK
        } else {
            godot::global::Error::FAILED
        }
    }
}
