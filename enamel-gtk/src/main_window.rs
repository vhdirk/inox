use std::rc::Rc;
use gio::ActionMapExt;
use gtk::GtkWindowExt;
use gtk;
use gio;
use glib;
use gtk::prelude::*;

use crossbeam_channel::Sender;
use failure::Error;
use rayon;
// use url::Url;

// use hammond_data::{dbqueries, Source};

use app::EnamelApp;
use app::Action;
// use utils::{itunes_to_rss, refresh};
use headerbar::HeaderBar;
use widgets::tag_list::{TagList, Msg as TagListMsg};


use relm::{Relm, Component, Update, Widget};
use relm::init as relm_init;


#[derive(Msg)]
pub enum Msg {
    Change,
    Quit,
}

#[derive(Clone)]
pub struct Model {
    app: Rc<EnamelApp>,
    content: String,
}

#[derive(Clone)]
struct Widgets {
    headerbar: Component<HeaderBar>,
    taglist: Component<TagList>
    //threadlist
    //threadview
}



// TODO: Factor out the hamburger menu
// TODO: Make a proper state machine for the headerbar states
pub struct MainWindow {
    model: Model,
    container: gtk::ApplicationWindow,
    widgets: Widgets
    // pub(crate) container: gtk::ApplicationWindow,
    // header: Rc<HeaderBar>
}

impl MainWindow {
    // fn new(ui: UI, application: gtk::Application) -> Self {
    //     let window = ui.builder.get_object("main_window")
    //                            .expect("Couldn't find main_window in ui file.");
    //     window.set_application(&model.gapp);

    // //     window
    // }

    // pub(crate) fn init(this: &Rc<Self>/*, sender: &Sender<Action>*/) {
    //     let weak = Rc::downgrade(this);

    //     //self.switch.set_stack(&content.get_stack());
    // }
}

impl Update for MainWindow{
    type Model = Model;
    type ModelParam = Rc<EnamelApp>;
    type Msg = Msg;

    fn model(relm: &Relm<Self>, app: Self::ModelParam) -> Model {
        Self::Model {
            app,
            content: String::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Change => {
                // self.model.content = self.widgets.input.get_text()
                //                                        .expect("get_text failed")
                //                                        .chars()
                //                                        .rev()
                //                                        .collect();
                // self.widgets.label.set_text(&self.model.content);
            },
            Quit => gtk::main_quit(),
        }
    }
}

impl Widget for MainWindow {
    type Root = gtk::ApplicationWindow;

    fn root(&self) -> Self::Root {
        self.container.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        
        let window = model.app.builder.get_object::<gtk::ApplicationWindow>("main_window")
                                  .expect("Couldn't find main_window in ui file.");
        window.set_application(&model.app.instance);


        let headerbar = relm_init::<HeaderBar>(model.app.clone()).unwrap(); 
        let taglist = relm_init::<TagList>(model.app.clone()).unwrap(); 

        MainWindow {
            model,
            container: window,
            widgets: Widgets{
                headerbar,
                taglist
            }
        }

    }

    fn init_view(&mut self) {

        let main_paned = self.model.app.builder.get_object::<gtk::Paned>("main_paned")
                                   .expect("Couldn't find main_paned in ui file.");

        let taglist_header = self.model.app.builder.get_object::<gtk::HeaderBar>("taglist_header")
                                 .expect("Couldn't find taglist_header in ui file.");

        // TODO: do I need to unbind this at some point?
        let _width_bind = main_paned.bind_property("position", &taglist_header, "width-request")
                                    .flags(glib::BindingFlags::SYNC_CREATE)
                                    .transform_to(move |_binding, value| {
                                        let offset = 6; //TODO: this offset was trial and error.
                                                        // we should calculate it somehow.
                                        return Some((value.get::<i32>().unwrap_or(0) + offset).to_value());
                                    })
                                    .build();

        self.container.show_all();

        self.widgets.taglist.emit(TagListMsg::Refresh);
    }

}