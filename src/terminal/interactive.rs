//! This module is the one containing all the code required for the "interactive" aspects of Tuit.
//! This includes code like structs for handling input, like [`interactive::MouseButton`] or
//! [`interactive::KeyState`], as well as the [`UpdateInfo`] and [`UpdateResult`] structs.

use core::time::Duration;
use crate::terminal::Rectangle;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// This enum represents the various buttons on the mouse.
pub enum MouseButton {
    /// The primary mouse button, usually the left click button.
    Primary,
    /// The secondary mouse button, usually the right click button.
    Secondary,
    /// Any auxiliary mouse buttons (for example, additional side buttons).
    AuxiliaryButton(u16),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
/// [`KeyState`] encompasses the current states that a keyboard key can be in (just pressed, currently held, and just released)
pub enum KeyState {
    /// Key has just been released
    KeyUp = 0,
    /// Key has just been pressed
    KeyDown = 1,
    /// Key is currently held
    KeyHeld = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
/// `UpdateInfo` encapsulates the information sent after an update
pub enum UpdateInfo {
    /// This event triggers when a cell (character) gets clicked. It includes the X co-ordinate,
    /// Y co-ordinate, and the mouse button that was clicked.
    ///
    /// <br>
    /// The variables are as follows:
    /// `CellClicked(x_coord, y_coord, mouse_button)`
    CellClicked(usize, usize, MouseButton),
    /// This can be sent to widgets to inform them of a printable keyboard key being
    /// pressed.
    KeyboardCharacter(char, KeyState),
    /// This can be sent to widgets to inform them of a non-printable keyboard key being pressed.
    ///
    /// Go to [the UEFI specification](https://uefi.org/specs/UEFI/2.10/Apx_B_Console.html) and
    /// check for USB keyboard HID values.
    KeyboardInput(u8, KeyState),
    /// This can be used to inform widgets of how much time has passed since they have
    /// last been updated.
    TimeDelta(Duration),
    /// This is used to inform widgets that the terminal has been resized so that they can
    /// re-calculate their dimensions or any other cached data reliant on the terminal's size.
    TerminalResized,
    #[default]
    /// This is used when there is no information to report to widgets that need to be updated.
    NoInfo,
}

impl UpdateInfo {
    /// Get the mouse position relative to a given [`Rectangle`].
    #[must_use]
    pub const fn mouse_relative_to(self, rect: Rectangle) -> Self {
        match self {
            Self::CellClicked(x, y, button)
            => {
                let Some(x) = x.checked_sub(rect.left()) else {
                    return Self::NoInfo
                };

                let Some(y) = y.checked_sub(rect.top()) else {
                    return Self::NoInfo
                };

                Self::CellClicked(x, y, button)
            }

            _ => self
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
/// `UpdateResult` includes information about the object's status. Objects are unable to destruct themselves,
/// and thus are reliant on the implementor to get rid of them once their lifecycle is over.
/// 
/// In the case that you are handling multiple widgets under the same [`Widget::update`](crate::widgets::Widget::update) method,
/// you can use ordering to determine which [`UpdateResult`] to return.
pub enum UpdateResult {
    /// No event has occurred, the object will continue to live.
    NoEvent = 0,
    /// The widget has been interacted with during the last update.
    /// (i.e. it has been clicked on, or a keystroke affected it).
    Interacted = 1,
    /// The object's lifecycle has ended, and it should now be destructured.
    LifecycleEnd = 2,
}
