/*
 * The Listeners to be implemented for the game logic.
 */

//= LISTENER TRAITS ================================================================================

pub trait WindowListener {

    fn on_close_requested(&self) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn on_keyboard_input(
        &self,
        device_id: &crate::window::DeviceId,
        state: crate::window::ElementState,
        virtual_keycode: crate::window::VirtualKeycode
    ) -> bool {
        true
    }
}


pub trait KeyboardListener {

}
