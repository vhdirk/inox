use std::rc::Rc;
use std::sync::Arc;

use cursive;
use cursive::direction::Orientation::Horizontal;
use cursive::event::{EventTrigger, Key};
use cursive::view::Identifiable;
use cursive::theme::{BaseColor, Color, Effect, Style, ColorStyle};
use cursive::{Cursive, Printer};
use cursive::views::{ViewRef, IdView, ListView, LinearLayout, ScrollView, OnEventView, TextView, EditView, BoxView};
use cursive::view::{View, ViewWrapper};
use cursive::utils::markup::StyledString;

use notmuch;

use enamel_core::database::Manager as DBManager;
use crate::lazy_list_view::LazyListView;

struct ThreadLineView {
    pub view: LinearLayout,
    pub thread: notmuch::Thread<'static, 'static>
}

impl ThreadLineView {
    pub fn new(thread: notmuch::Thread<'static, 'static>) -> Self {

        let tags: Vec<String> = thread.tags().collect();
        let mut view = LinearLayout::new(Horizontal)
            .child(TextView::new(tags.join(" ")))
            // .child(TextView::new(StyledString::styled(tags.join(" "), Color::Dark(BaseColor::Blue))))
            .child(TextView::new(thread.subject()))
            .child(EditView::new());

        Self {
            view,
            thread
        }
    }

}

impl ViewWrapper for ThreadLineView {
    cursive::wrap_impl!(self.view: LinearLayout);

    fn wrap_draw(&self, printer: &Printer<'_, '_>) {
        
        printer.with_color(if printer.focused { ColorStyle::highlight() } else { ColorStyle::primary() }, |printer| {
            self.with_view(|v| v.draw(printer));
        });        
    }
}

pub struct ThreadListView{
    pub view: LazyListView

}


impl ThreadListView {

    cursive::inner_getters!(self.view: LazyListView);

    pub fn new(dbmanager: Rc<DBManager>) -> Self {

        Self {
            view: LazyListView::new()
        }
    }

    pub fn set_query(&mut self, query: Arc<notmuch::Query<'static>>){

        let threads = <notmuch::Query as notmuch::QueryExt>::search_threads(query).unwrap();

        for thread in threads {
            self.view.add_child(&thread.id().to_string(), ThreadLineView::new(thread));
        }

    }

}


impl ViewWrapper for ThreadListView {
    cursive::wrap_impl!(self.view: LazyListView);



}
