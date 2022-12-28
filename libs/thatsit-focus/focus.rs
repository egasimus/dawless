#![feature(unboxed_closures, fn_traits)]

use std::{io::Result, slice::Iter, slice::IterMut};
use thatsit::{*, crossterm::event::Event};

/// The focus state of an item
#[derive(Debug, Default)]
pub struct Focus<T>(
    /// Whether this item is focused
    bool,
    /// Whether an item owned by this item is focused
    Option<T>
);

pub trait FocusList<T> {
    /// Get an immutable reference to the list of items
    fn items (&self) -> &Vec<T>;
    /// Get a mutable reference to the list of items
    fn items_mut (&mut self) -> &mut Vec<T>;
    /// Get an immutable reference to the focus state
    fn state (&self) -> &Focus<usize>;
    /// Get a mutable reference to the focus state
    fn state_mut (&mut self) -> &mut Focus<usize>;

    /// Iterate over immutable references to the contained items
    fn iter (&self) -> Iter<T> {
        self.items().iter()
    }
    /// Iterate over immutable references to the contained items
    fn iter_mut (&mut self) -> IterMut<T> {
        self.items_mut().iter_mut()
    }
    /// Iterate over mutable references to the contained items
    /// Replace the list of items, resetting the item focus
    fn replace (&mut self, items: Vec<T>) {
        *self.items_mut() = items;
        self.state_mut().1 = None;
    }
    /// Count the contained items
    fn len (&self) -> usize {
        self.items().len()
    }
    /// Get an immutable reference the currently focused item
    fn get (&self) -> Option<&T> {
        match self.state().1 {
            Some(i) => self.items().get(i),
            _ => None
        }
    }
    /// Get a mutable reference the currently focused item
    fn get_mut (&mut self) -> Option<&mut T> {
        match self.state().1 {
            Some(i) => self.items_mut().get_mut(i),
            _ => None
        }
    }
    /// Set the focus
    fn focus (&mut self) -> bool {
        self.state_mut().0 = true;
        true
    }
    /// Clear the focus
    fn unfocus (&mut self) -> bool {
        self.state_mut().0 = false;
        true
    }
    /// Get the index of the currently selected item
    fn selected (&self) -> Option<usize> {
        self.state().1
    }
    /// Set the selected item
    fn select (&mut self, index: usize) -> bool {
        if self.items().get(index).is_some() {
            self.state_mut().1 = Some(index);
            true
        } else {
            false
        }
    }
    /// Select the next item
    fn select_next (&mut self) -> bool {
        self.unselect();
        if let Some(index) = self.state().1 {
            self.select(if index >= self.items().len() - 1 { 0 } else { index + 1 })
        } else {
            self.select(0)
        }
    }
    /// Select the previous item
    fn select_prev (&mut self) -> bool {
        self.unselect();
        if let Some(index) = self.state().1 {
            self.select(if index == 0 { self.items().len() - 1 } else { index - 1 })
        } else {
            self.select(0)
        }
    }
    /// Clear the selected item
    fn unselect (&mut self) -> bool {
        self.state_mut().1 = None;
        true
    }
}

/// A list of sequentially selectable items
#[derive(Debug)]
pub struct FocusState<T> {
    /// The list of items
    items: Vec<T>,
    /// The focus state
    pub state: Focus<usize>,
}

impl<T> Default for FocusState<T> {
    /// Create an empty focus list
    fn default () -> Self { Self { items: vec![], state: Focus(false, None) } }
}

impl<T> FocusState<T> {
    /// Create a new focus list, taking ownership of a collection of items
    pub fn new (items: Vec<T>) -> Self { Self { items, state: Focus(false, None) } }
}

impl<T> FocusList<T> for FocusState<T> {
    fn items (&self) -> &Vec<T> { &self.items }
    fn items_mut (&mut self) -> &mut Vec<T> { &mut self.items }
    fn state (&self) -> &Focus<usize> { &self.state }
    fn state_mut (&mut self) -> &mut Focus<usize> { &mut self.state }
}

/// A vertical list of focusable items
#[derive(Debug, Default)]
pub struct FocusColumn<T: TUI> {
    /// A focus list of the contained widgets
    pub items:  FocusState<T>,
    /// A scroll offset
    pub offset: usize
}

impl<T: TUI> FocusColumn<T> {
    pub fn new (items: Vec<T>) -> Self { Self { items: FocusState::new(items), offset: 0 } }
}

impl<T: TUI> FocusList<T> for FocusColumn<T> {
    fn items (&self) -> &Vec<T> { &self.items.items }
    fn items_mut (&mut self) -> &mut Vec<T> { &mut self.items.items }
    fn state (&self) -> &Focus<usize> { &self.items.state }
    fn state_mut (&mut self) -> &mut Focus<usize> { &mut self.items.state }
}

impl<T: TUI> TUI for FocusColumn<T> {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> {
        Ok(col_stretch(|add|{ for item in self.items.items.iter() { add(&*item); } }))
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(match self.get_mut() {
            Some(item) => item.handle(event),
            None => Ok(false)
        }? || match_key!((event) {
            KeyCode::Up   => { self.select_prev() },
            KeyCode::Down => { self.select_next() }
        }))
    }
}

/// A horizontal list of focusable items
#[derive(Debug, Default)]
pub struct FocusRow<T: TUI> {
    /// A focus list of the contained widgets
    pub items: FocusState<T>,
    /// A scroll offset
    pub offset: usize
}

impl<T: TUI> FocusList<T> for FocusRow<T> {
    fn items (&self) -> &Vec<T> { &self.items.items }
    fn items_mut (&mut self) -> &mut Vec<T> { &mut self.items.items }
    fn state (&self) -> &Focus<usize> { &self.items.state }
    fn state_mut (&mut self) -> &mut Focus<usize> { &mut self.items.state }
}

impl<T: TUI> FocusRow<T> {
    pub fn new (items: Vec<T>) -> Self { Self { items: FocusState::new(items), offset: 0 } }
}

impl<T: TUI> TUI for FocusRow<T> {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> {
        Ok(row(|add|{ for item in self.items.items.iter() { add(&*item); } }))
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(match self.get_mut() {
            Some(item) => item.handle(event),
            None => Ok(false)
        }? || match_key!((event) {
            KeyCode::Up   => { self.select_prev() },
            KeyCode::Down => { self.select_next() }
        }))
    }
}

/// A stack of focusable items, rendering one at a time
#[derive(Debug, Default)]
pub struct FocusStack<T: TUI>(pub FocusState<T>);

impl<T: TUI> FocusList<T> for FocusStack<T> {
    fn items (&self) -> &Vec<T> { &self.0.items }
    fn items_mut (&mut self) -> &mut Vec<T> { &mut self.0.items }
    fn state (&self) -> &Focus<usize> { &self.0.state }
    fn state_mut (&mut self) -> &mut Focus<usize> { &mut self.0.state }
}

impl<T: TUI> FocusStack<T> {
    pub fn new (items: Vec<T>) -> Self { Self(FocusState::new(items)) }
}

impl<T: TUI> TUI for FocusStack<T> {
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        match self.get() { Some(item) => item.layout(max), None => Ok(BLANK.into()) }
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        match self.get_mut() {
            Some(item) => item.handle(event),
            None => Ok(false)
        }
    }
}
