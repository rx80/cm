pub mod keycodes;

use ncurses::*;
use std::cmp::{min, max};
use keycodes::*;

fn clamp<T: Ord>(x: T, low: T, high: T) -> T {
    min(max(low, x), high)
}

pub trait RenderItem {
    fn render(&self, row: Row, cursor_x: usize,
              selected: bool, focused: bool);
}

pub struct ItemList<Item> {
    pub items: Vec<Item>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl<Item> ItemList<Item> where Item: RenderItem {
    pub fn up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1
        }
    }

    pub fn down(&mut self) {
        if self.cursor_y + 1 < self.items.len() {
            self.cursor_y += 1;
        }
    }

    pub fn left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    pub fn right(&mut self) {
        self.cursor_x += 1;
    }

    pub fn delete_current(&mut self) {
        self.items.remove(self.cursor_y);
        self.cursor_y = clamp(self.cursor_y, 0, self.items.len() - 1);
    }

    pub fn handle_key(&mut self, key: i32) {
        match key {
            KEY_S  => self.down(),
            KEY_W  => self.up(),
            KEY_D  => self.right(),
            KEY_A  => self.left(),
            KEY_DC => self.delete_current(),
            _ => {}
        }
    }

    pub fn render(&self, Rect {x, y, w, h}: Rect, focused: bool) {
        if h > 0 {
            // TODO(#16): word wrapping for long lines
            for (i, item) in self.items.iter().skip(self.cursor_y / h * h).enumerate().take_while(|(i, _)| *i < h) {
                item.render(Row {x: x, y: i + y, w: w}, self.cursor_x,
                            i == (self.cursor_y % h),
                            focused);
            }
        }
    }

    pub fn current_item(&self) -> &Item {
        &self.items[self.cursor_y]
    }
}

pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

pub struct Row {
    pub x: usize,
    pub y: usize,
    pub w: usize,
}

pub struct EditField {
    pub data : String,
    pub cursor_x : usize,
}

impl EditField {
    pub fn render(&self, Row {x, y, w}: Row) {
        let begin = self.cursor_x / w * w;
        let end   = usize::min(begin + w, self.data.len());
        mv(y as i32, x as i32);
        for _ in 0..w {
            addstr(" ");
        }
        mv(y as i32, x as i32);
        addstr(&self.data.get(begin..end).unwrap_or(""));
        mv(y as i32, (x + self.cursor_x % w) as i32);
    }

    pub fn handle_key(&mut self, key: i32) {
        if 32 <= key && key <= 126 {
            self.data.insert(self.cursor_x, key as u8 as char);
            self.cursor_x += 1;
        }

        match key {
            KEY_RIGHT     if self.cursor_x < self.data.len() => self.cursor_x += 1,
            KEY_LEFT      if self.cursor_x > 0               => self.cursor_x -= 1,
            KEY_BACKSPACE if self.cursor_x > 0               => {
                self.cursor_x -= 1;
                self.data.remove(self.cursor_x);
            }
            _ => {}
        }
    }
}
