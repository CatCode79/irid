
//= ENUM ALIASES ===================================================================================

/// It's an alias to winit::event::ElementState.
pub type ElementState = winit::event::ElementState;

///Describes the reason the event loop is resuming.
pub type StartCause = winit::event::StartCause;

/// It's an alias to winit::event::VirtualKeyCode.
pub type VirtualKeyCode = winit::event::VirtualKeyCode;


//= STRUCT ALIASES =================================================================================

/// It's an alias to winit::event::DeviceId
pub type DeviceId = winit::event::DeviceId;


//= STRUCT WRAPPERS ================================================================================

/// Describes a keyboard input event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyboardInput {
    /// Describes the input state of a key.
    pub state: ElementState,

    /// Identifies the semantic meaning of the key.
    pub virtual_keycode: VirtualKeyCode,
}
