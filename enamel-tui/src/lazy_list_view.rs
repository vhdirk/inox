use std::rc::Rc;
use std::sync::Arc;

use cursive;
use cursive::direction::Orientation::Horizontal;
use cursive::direction::Direction;
use cursive::event::{AnyCb, Event, EventTrigger, Key, EventResult};

use cursive::Cursive;
use cursive::views::{ViewRef, IdView, LinearLayout, ScrollView, OnEventView};
use cursive::view::{View, Identifiable, ViewWrapper, Selector, IntoBoxedView};
use cursive::With;

use crate::list_view::{ListView, ListChild};



pub struct LazyListView{
    pub view: ScrollView<ListView>

}


impl LazyListView {

    /// Creates a new, empty `LazyListView`.
    pub fn new() -> Self {
        Self {
            view: ScrollView::new(ListView::new())
        }
    }

    /// Returns the number of children, including delimiters.
    pub fn len(&self) -> usize {
        self.view.get_inner().len()
    }

    /// Returns `true` if this view contains no children.
    ///
    /// Returns `false` if at least a delimiter or a view is present.
    pub fn is_empty(&self) -> bool {
        self.view.get_inner().is_empty()
    }

    /// Returns a reference to the children
    pub fn children(&self) -> &[ListChild] {
        self.view.get_inner().children()
    }

    /// Returns a reference to the child at the given position.
    pub fn get_row(&self, id: usize) -> &ListChild {
        self.view.get_inner().get_row(id)
    }

    /// Gives mutable access to the child at the given position.
    ///
    /// # Panics
    ///
    /// Panics if `id >= self.len()`.
    pub fn row_mut(&mut self, id: usize) -> &mut ListChild {
        self.view.get_inner_mut().row_mut(id)
    }

    /// Adds a view to the end of the list.
    pub fn add_child<V: View + 'static>(
        &mut self,
        label: &str,
        view: V,
    ) {
        self.view.get_inner_mut().add_child(label, view)
    }

    /// Removes all children from this view.
    pub fn clear(&mut self) {
        self.view.get_inner_mut().clear()
    }

    /// Adds a view to the end of the list.
    ///
    /// Chainable variant.
    pub fn child<V: View + 'static>(
        self,
        label: &str,
        view: V,
    ) -> Self {
        self.with(|s| s.add_child(label, view))
    }

    /// Adds a delimiter to the end of the list.
    pub fn add_delimiter(&mut self) {
        self.view.get_inner_mut().add_delimiter();
    }

    /// Adds a delimiter to the end of the list.
    ///
    /// Chainable variant.
    pub fn delimiter(self) -> Self {
        self.with(Self::add_delimiter)
    }

    /// Removes a child from the view.
    ///
    /// # Panics
    ///
    /// If `index >= self.len()`.
    pub fn remove_child(&mut self, index: usize) -> ListChild {
        self.view.get_inner_mut().remove_child(index)
    }

    /// Sets a callback to be used when an item is selected.
    pub fn set_on_select<F>(&mut self, cb: F)
    where
        F: Fn(&mut Cursive, &String) + 'static,
    {
        self.view.get_inner_mut().set_on_select(cb)
    }

    /// Sets a callback to be used when an item is selected.
    ///
    /// Chainable variant.
    pub fn on_select<F>(self, cb: F) -> Self
    where
        F: Fn(&mut Cursive, &String) + 'static,
    {
        self.with(|s| s.set_on_select(cb))
    }

    /// Returns the index of the currently focused item.
    ///
    /// Panics if the list is empty.
    pub fn focus(&self) -> usize {
        self.view.get_inner().focus()
    }


    pub fn move_focus(
        &mut self,
        n: usize,
        source: Direction,
    ) -> EventResult {
        self.view.get_inner_mut().move_focus(n, source)
    }

}


impl View for LazyListView {

    fn draw(&self, printer: &cursive::Printer<'_, '_>) {
        self.view.draw(&printer)
    }

    fn layout(&mut self, size: cursive::Vec2) {
        self.view.layout(size)
    }

    fn needs_relayout(&self) -> bool {
        self.view.needs_relayout()
    }

    fn required_size(&mut self, constraint: cursive::Vec2) -> cursive::Vec2 {
        self.view.required_size(constraint)
    }

    fn on_event(&mut self, event: Event) -> EventResult {

        match event {
            Event::Char('j') | Event::Key(Key::Down) => {
                self.move_focus(1, Direction::up())
            },
            Event::Char('k') | Event::Key(Key::Up) => {
                self.move_focus(1, Direction::down())
            },
            Event::Key(Key::PageUp) => {
                self.move_focus(10, Direction::down())
            }
            Event::Key(Key::PageDown) => {
                self.move_focus(10, Direction::up())
            }

            // EventResult::with_cb(|siv| {


            // })

            _ => self.view.on_event(event)
        }


    }

    fn call_on_any<'a>(&mut self, selector: &Selector<'_>, callback: AnyCb<'a>) {
        self.view.call_on_any(&selector, callback)
    }

    fn focus_view(&mut self, selector: &Selector<'_>) -> Result<(), ()> {
        self.view.focus_view(&selector)
    }

    fn take_focus(&mut self, source: Direction) -> bool {
        self.view.take_focus(source)
    }

    fn important_area(&self, view_size: cursive::Vec2) -> cursive::Rect {
        self.view.important_area(view_size)
    }

}
