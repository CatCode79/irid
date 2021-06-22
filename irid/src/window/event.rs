
//= ENUM ALIASES ===================================================================================

/// Identifier for a specific analog axis on some device.
///
/// Type alias of [winit::event::AxisId](winit::event::AxisId).
pub type AxisId = winit::event::AxisId;

/// Describes the input state of a key.
///
/// Type alias of [winit::event::ElementState](winit::event::ElementState).
pub type ElementState = winit::event::ElementState;

/// Describes a button of a mouse controller.
///
/// Type alias of [winit::event::MouseButton](winit::event::MouseButton).
pub type MouseButton = winit::event::MouseButton;

/// Describes a difference in the mouse scroll wheel state.
///
/// Type alias of [winit::event::MouseScrollDelta](winit::event::MouseScrollDelta).
pub type MouseScrollDelta = winit::event::MouseScrollDelta;

/// Describes the reason the event loop is resuming.
///
/// Type alias of [winit::event::StartCause](winit::event::StartCause).
pub type StartCause = winit::event::StartCause;

/// Describes touch-screen input state.
///
/// Type alias of [winit::event::TouchPhase](winit::event::TouchPhase).
pub type TouchPhase = winit::event::TouchPhase;

/// Symbolic name for a keyboard key.
///
/// Type alias of [winit::event::VirtualKeyCode](winit::event::VirtualKeyCode).
pub type VirtualKeyCode = winit::event::VirtualKeyCode;


//= STRUCT ALIASES =================================================================================

/// It's an alias to winit::event::DeviceId.
///
/// Type alias of [winit::event::DeviceId](winit::event::DeviceId).
pub type DeviceId = winit::event::DeviceId;

/// Represents the current state of the keyboard modifiers.
///
/// Type alias of [winit::event::ModifiersState](winit::event::ModifiersState).
pub type ModifiersState = winit::event::ModifiersState;


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
