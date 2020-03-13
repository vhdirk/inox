use std::rc::Rc;
use std::sync::Arc;

use log::*;
use cursive;
use cursive::direction::Orientation::Horizontal;
use cursive::direction::Direction;
use cursive::event::{AnyCb, Callback, Event, EventTrigger, Key, EventResult};

use cursive::Cursive;
use cursive::views::{ViewRef, IdView, LinearLayout, ScrollView, OnEventView};
use cursive::view::{View, Identifiable, ViewWrapper, Selector, IntoBoxedView};
use cursive::With;

use crate::list_view::{ListView, ListChild};

// type LoadCallback = Rc<Box<dyn Fn(&mut Cursive, &mut dyn LazyViewWrapper) -> Option<EventResult>>>;


#[macro_export]
macro_rules! wrap_lazy_impl {
    (self.$v:ident: $t:ty) => {
        type L = $t;

        fn with_lazy_view<F, R>(&self, f: F) -> Option<R>
            where F: FnOnce(&Self::L) -> R
        {
            Some(f(&self.$v))
        }

        fn with_lazy_view_mut<F, R>(&mut self, f: F) -> Option<R>
            where F: FnOnce(&mut Self::L) -> R
        {
            Some(f(&mut self.$v))
        }

        // fn into_lazy_inner(self) -> Result<Self::L, Self> where Self::L: Sized {
        //     Ok(self.$v)
        // }
    };
}


pub trait LazyView: View {
    fn load_data(&mut self) -> EventResult;
}

pub trait LazyViewWrapper: ViewWrapper{
    type L: LazyView + ?Sized;

    /// Runs a function on the inner view, returning the result.
    ///
    /// Returns `None` if the inner view is unavailable.  This should only
    /// happen with some views if they are already borrowed by another call.
    fn with_lazy_view<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Self::L) -> R;

    /// Runs a function on the inner view, returning the result.
    ///
    /// Returns `None` if the inner view is unavailable.  This should only
    /// happen with some views if they are already borrowed by another call.
    fn with_lazy_view_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Self::L) -> R;


    fn wrap_load_data(&mut self) -> EventResult {
        debug!("LazyViewWrapper::wrap_load_data");
        self.with_lazy_view_mut(|v| v.load_data())
            .unwrap_or_else(|| EventResult::Ignored)
    }
}

impl<T: LazyViewWrapper> LazyView for T {

    fn load_data(&mut self) -> EventResult {
        debug!("LazyView for T::load_data");
        self.wrap_load_data()
    }
}


pub struct LazyListView{
    pub view: ScrollView<ListView>,
    on_load: Option<Rc<dyn Fn(&mut Cursive)>>,
}


impl LazyListView {

    /// Creates a new, empty `LazyListView`.
    pub fn new() -> Self {
        Self {
            view: ScrollView::new(ListView::new()),
            on_load: None
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


    /// Sets a callback to be used when an item is selected.
    pub fn set_on_load<F>(&mut self, cb: F)
    where
        F: Fn(&mut Cursive) + 'static,
    {
        self.on_load = Some(Rc::new(cb));
    }

    /// Sets a callback to be used when an item is selected.
    ///
    /// Chainable variant.
    pub fn on_load<F>(self, cb: F) -> Self
    where
        F: Fn(&mut Cursive) + 'static,
    {
        self.with(|s| s.set_on_load(cb))
    }

}



// impl ViewWrapper for LazyListView {
//     cursive::wrap_impl!(self.view: ScrollView<ListView>);


//     fn wrap_on_event(&mut self, event: Event) -> EventResult{
//         debug!("LazyListView::wrap_on_event");
//         let ret = match event {
//             Event::Char('k') | Event::Key(Key::Up) => {
//                 self.move_focus(1, Direction::down())
//             },
//             Event::Char('j') | Event::Key(Key::Down) => {
//                 let ret = self.move_focus(1, Direction::up());
//                 if self.view.is_at_bottom() {
//                     return self.load_data()
//                 }

//                 ret
//             },
//             Event::Key(Key::PageUp) => {
//                 self.move_focus(10, Direction::down())
//             },
//             Event::Key(Key::PageDown) => {
//                 let ret = self.move_focus(10, Direction::up());

//                 return self.load_data();
//                 ret;
//             },



//             // EventResult::with_cb(|siv| {


//             // })

//             _ => self.view.on_event(event)
//         };

//         // debug!("self.view.is_at_bottom() {:?}", self.view.is_at_bottom());
//         // if self.view.is_at_bottom() || self.view.is_at_top() {
//         //     return self.load_data()
//         // }
//         ret


//     }

// }

// impl LazyViewWrapper for LazyListView {
//     wrap_lazy_impl!(self.view: ScrollView<ListView>);

// }

impl LazyView for ScrollView<ListView>{
    fn load_data(&mut self) -> EventResult {
        debug!("LazyScrollView::load_data");
        EventResult::Ignored
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
        debug!("LazyListView::on_event");
        let ret = match event {
            Event::Char('k') | Event::Key(Key::Up) => {
                self.move_focus(1, Direction::down())
            },
            Event::Char('j') | Event::Key(Key::Down) => {
                let ld = if self.view.is_at_bottom() {
                    self.load_data()
                } else {
                    EventResult::Ignored
                };
                ld.and(self.move_focus(1, Direction::up())).and(self.view.on_event(event))
            },
            Event::Key(Key::PageUp) => {
                self.move_focus(10, Direction::down())
            },
            Event::Key(Key::PageDown) => {
                let ld = if self.view.is_at_bottom() {
                    self.load_data()
                } else {
                    EventResult::Ignored
                };
                ld.and(self.move_focus(1, Direction::up())).and(self.view.on_event(event))
            },
            Event::Mouse{..} => {
                let ld = if self.view.is_at_bottom() {
                    self.load_data()
                } else {
                    EventResult::Ignored
                };
                ld.and(self.view.on_event(event))
            }



            // EventResult::with_cb(|siv| {


            // })

            _ => self.view.on_event(event)
        };

        // debug!("self.view.is_at_bottom() {:?}", self.view.is_at_bottom());
        // if self.view.is_at_bottom() || self.view.is_at_top() {
        //     return self.load_data()
        // }
        ret


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

// impl LazyView for LazyListView {
//     fn load_data(&mut self) -> EventResult {
//         debug!("LazyListView::load_data");
//         EventResult::Consumed(self.on_load.clone().map(|cb| {
//             Callback::from_fn(move |s| cb(s))
//         }))
//     }

// }

use std::cell::RefCell;
impl LazyView for LazyListView {
    fn load_data(&mut self) -> EventResult {
        debug!("LazyListView::load_data");

        EventResult::Consumed(self.on_load.clone().map(|cb| {
            Callback::from_fn(move |s| cb(s))
        }))
    }

}

impl<T: LazyView + 'static> LazyViewWrapper for IdView<T> {
    type L = T;

    fn with_lazy_view<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Self::L) -> R
    {
        self.with_view(f)
    }

    /// Runs a function on the inner view, returning the result.
    ///
    /// Returns `None` if the inner view is unavailable.  This should only
    /// happen with some views if they are already borrowed by another call.
    fn with_lazy_view_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Self::L) -> R
    {
        Some(f(&mut self.get_mut()))
    }

}
