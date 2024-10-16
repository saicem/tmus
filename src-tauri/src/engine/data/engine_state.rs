/// Represents the different states of the engine.
///
/// # States
/// - `Running`: Indicates the engine is currently running.
/// - `Suspended`: Engine is suspended. Won't accept any focus event.
/// - `Busy`: Indicates that writing is not possible, and focus events are blocked until this state ends.
///   Typically used during operations that change base data, like optimizing file storage.
pub enum EngineState {
    Running,
    Suspended,
    Busy,
}
