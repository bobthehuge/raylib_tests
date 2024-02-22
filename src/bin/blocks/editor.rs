use crate::*;
use raylib::prelude::*;

pub struct Editor {
    mode: EditorMode,
    next_mode: EditorMode,
    locked: bool,
}

#[derive(PartialEq, Clone, Copy)]
pub enum EditorMode {
    Init,
    Edit,
    Render,
    Exit,
    None,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            mode: EditorMode::Init,
            next_mode: EditorMode::Render,
            locked: false,
        }
    }

    #[inline]
    pub fn get_mode(&self) -> EditorMode {
        self.mode
    }

    #[inline]
    pub fn set_mode(&mut self, mode: EditorMode) { self.mode = mode }

    #[inline]
    pub fn set_next_mode(&mut self, mode: EditorMode) { self.next_mode = mode }

    #[inline]
    pub fn set_modes(&mut self, mode: EditorMode, next_mode: EditorMode) {
        self.mode = mode;
        self.next_mode = next_mode;
    }

    #[inline]
    pub fn repeat_mode(&mut self) { self.next_mode = self.mode }

    #[inline]
    pub fn is_mode(&self, mode: EditorMode) -> bool { self.mode == mode }

    #[inline]
    pub fn is_next_mode(&self, mode: EditorMode) -> bool { self.next_mode == mode }

    #[inline]
    pub fn is_mode_locked(&self) -> bool { self.locked }

    #[inline]
    pub fn lock_mode(&mut self) { self.locked = true }

    #[inline]
    pub fn unlock_mode(&mut self) { self.locked = false }

    pub fn cycle_mode(&mut self) {
        if self.is_mode_locked() {
            self.repeat_mode();
        } else {
            self.mode = self.next_mode;
            self.next_mode = EditorMode::None;
        }
    }
}
