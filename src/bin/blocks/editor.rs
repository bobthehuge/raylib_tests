use crate::*;
use raylib::prelude::*;

pub struct Editor {
    mode: EditorMode,
    next_mode: EditorMode,
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
        }
    }

    #[inline]
    pub fn get_mode(&self) -> EditorMode {
        self.mode
    }

    #[inline]
    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode
    }

    #[inline]
    pub fn edit_mode(&mut self) {
        self.next_mode = EditorMode::Edit
    }

    #[inline]
    pub fn is_mode_edit(&self) -> bool {
        self.mode == EditorMode::Edit
    }

    #[inline]
    pub fn is_next_mode_edit(&self) -> bool {
        self.mode == EditorMode::Edit
    }

    #[inline]
    pub fn render_mode(&mut self) {
        self.mode = EditorMode::Render
    }

    #[inline]
    pub fn is_mode_render(&self) -> bool {
        self.mode == EditorMode::Render
    }

    #[inline]
    pub fn exit_mode(&mut self) {
        self.mode = EditorMode::Exit
    }

    #[inline]
    pub fn is_mode_exit(&self) -> bool {
        self.mode == EditorMode::Exit
    }
}
