use std::rc::Rc;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::Arc;

use log::*;

use cursive;
use cursive::direction::Direction;
use cursive::direction::Orientation::Horizontal;
use cursive::event::{EventTrigger, Key, EventResult};
use cursive::view::Identifiable;
use cursive::theme::{BaseColor, Color, Effect, Style, ColorStyle};
use cursive::{Cursive, Printer};
use cursive::views::{ViewRef, IdView, ListView, LinearLayout, ScrollView, OnEventView, TextView, EditView, BoxView};
use cursive::view::{View, ViewWrapper};
use cursive::utils::markup::StyledString;

use notmuch;

use enamel_core::database::Manager as DBManager;
use crate::lazy_list_view::{LazyListView, LazyView, LazyViewWrapper};
use crate::wrap_lazy_impl;




struct ThreadLineView {
    pub view: LinearLayout,
    pub thread: notmuch::Thread<'static, 'static>
}

impl ThreadLineView {
    pub fn new(thread: notmuch::Thread<'static, 'static>) -> Self {

        let tags: Vec<String> = thread.tags().collect();
        let view = LinearLayout::new(Horizontal)
            //.child(TextView::new(tags.join(" ")))
            //
            
            .child(TextView::new(StyledString::styled(tags.join(" "), Color::Dark(BaseColor::Blue))))
            .child(TextView::new(thread.subject()));

        Self {
            view,
            thread
        }
    }

}

impl ViewWrapper for ThreadLineView {
    cursive::wrap_impl!(self.view: LinearLayout);

    fn wrap_draw(&self, printer: &Printer<'_, '_>) {
        
        let color = if printer.focused {
            ColorStyle::highlight()
        } else {
            ColorStyle::primary()
        };

        printer.with_color(color, |printer| {
            self.with_view(|v| v.draw(printer));
        });        
    }

    // always focusable
    fn wrap_take_focus(&mut self, source: Direction) -> bool {
        true
    }

}

pub struct ThreadListView{
    pub view: IdView<LazyListView>,
    threads: Option<Rc<RefCell<notmuch::Threads<'static, 'static>>>>

}



// fn do_lazy_load(siv: &mut Cursive, view: &mut LazyListView){


// }


impl ThreadListView {

    cursive::inner_getters!(self.view: IdView<LazyListView>);

    pub fn new(dbmanager: Rc<DBManager>) -> Self {

        let mut view = LazyListView::new().with_id("ThreadListView");


        Self {
            view,
            threads: None
        }
    }

    pub fn set_query(&mut self, query: Arc<notmuch::Query<'static>>){

        let o_threads = <notmuch::Query as notmuch::QueryExt>::search_threads(query).unwrap();
        self.threads = Some(Rc::new(RefCell::new(o_threads)));
        
        let threads = self.threads.clone();

        // self.load_data();
        self.view.with_view_mut(|v| v.set_on_load(move |siv|{
            let mt = threads.as_ref().unwrap();
            let mut num = 0;
            while let Some(thread) = mt.borrow_mut().next() {
                siv.call_on_id("ThreadListView", |v: &mut LazyListView| v.add_child(&thread.id().to_string(), ThreadLineView::new(thread)));

                // );

                if num == 10 {
                    break;
                }
                num += 1;
            }
        }));

        // let mut num = 0;
        // while let Some(thread) = threads.next() {
        //     self.view.add_child(&thread.id().to_string(), ThreadLineView::new(thread));

        //     if num == 10 {
        //         break;
        //     }
        //     num += 1;
        // }


    }

}


impl ViewWrapper for ThreadListView {
    cursive::wrap_impl!(self.view: IdView<LazyListView>);

}

// impl LazyViewWrapper for ThreadListView {
//     wrap_lazy_impl!(self.view: IdView<LazyListView>);

//     fn wrap_load_data(&mut self) -> EventResult {
//         debug!("ThreadListView::wrap_load_data");
//         if self.threads.is_none() {
//             return EventResult::Ignored;
//         }
//         // EventResult::with_cb(|siv| {
//         let mt = self.threads.as_ref().unwrap();
//         let mut num = 0;
//         while let Some(thread) = mt.borrow_mut().next() {
//             self.with_view_mut(|v| v.with_view_mut(|m| m.add_child(&thread.id().to_string(), ThreadLineView::new(thread))));

//             if num == 10 {
//                 break;
//             }
//             num += 1;
//         }
//         // })
//         EventResult::Consumed(None)
//         // self.view.set_on_load(move |siv, tlv|{
//         //     let mut num = 0;
//         //     
//         // });

//     }

// }
