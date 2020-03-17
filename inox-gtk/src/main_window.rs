use std::cell::RefCell;
use std::sync::Arc;

use gtk::GtkWindowExt;
use gtk;
use glib;
use futures::future::FutureExt;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Sender;
use glib::{glib_wrapper, glib_object_wrapper, glib_object_subclass, glib_object_impl};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;


use log::*;




use crate::get_widget;
use crate::app::{Action, InoxApplication, InoxApplicationPrivate};

// use crate::headerbar::HeaderBar;
use inox_core::database::Thread;

use crate::components::thread_list::ThreadList;
use crate::components::thread_view::ThreadView;

pub struct MainWindowPrivate {
    window_builder: gtk::Builder,
    // menu_builder: gtk::Builder,

    thread_list: RefCell<Option<ThreadList>>,
    thread_view: RefCell<Option<ThreadView>>

    // current_notification: RefCell<Option<Rc<Notification>>>,
}



impl ObjectSubclass for MainWindowPrivate {
    const NAME: &'static str = "inox_MainWindow";
    type ParentType = gtk::ApplicationWindow;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn new() -> Self {

        // static_resource::new_builder().get_object::<gtk::ApplicationWindow>("main_window");
        let window_builder = gtk::Builder::new_from_resource("/com/github/vhdirk/Inox/gtk/main_window.ui");

        Self {
            window_builder,
            thread_list: RefCell::new(None),
            thread_view: RefCell::new(None)

        }
    }
}

// Implement GLib.Object for MainWindow
impl ObjectImpl for MainWindowPrivate {
    glib_object_impl!();
}

// Implement Gtk.Widget for MainWindow
impl WidgetImpl for MainWindowPrivate {}

// Implement Gtk.Container for MainWindow
impl ContainerImpl for MainWindowPrivate {}

// Implement Gtk.Bin for MainWindow
impl BinImpl for MainWindowPrivate {}

// Implement Gtk.Window for MainWindow
impl WindowImpl for MainWindowPrivate {}

// Implement Gtk.ApplicationWindow for MainWindow
impl ApplicationWindowImpl for MainWindowPrivate {}


// Wrap MainWindowPrivate into a usable gtk-rs object
glib_wrapper! {
    pub struct MainWindow(
        Object<subclass::simple::InstanceStruct<MainWindowPrivate>,
        subclass::simple::ClassStruct<MainWindowPrivate>,
        MainWindowClass>)
        @extends gtk::Widget, gtk::Container, gtk::Bin, gtk::Window, gtk::ApplicationWindow;

    match fn {
        get_type => || MainWindowPrivate::get_type().to_glib(),
    }
}

// MainWindow implementation itself
impl MainWindow {
    pub fn new(sender: Sender<Action>, app: InoxApplication) -> Self {
        // Create new GObject and downcast it into MainWindow
        let window = glib::Object::new(MainWindow::static_type(), &[]).unwrap().downcast::<MainWindow>().unwrap();

        app.add_window(&window.clone());
        window.setup_widgets(sender.clone());
        window.setup_signals();
        window.setup_gactions(sender);
        window
    }

    pub fn setup_widgets(&self, sender: Sender<Action>) {
        let self_ = MainWindowPrivate::from_instance(self);
        let app: InoxApplication = self.get_application().unwrap().downcast::<InoxApplication>().unwrap();
        let _app_private = InoxApplicationPrivate::from_instance(&app);

        // Add headerbar/content to the window itself
        get_widget!(self_.window_builder, gtk::Paned, main_header);
        self.set_titlebar(Some(&main_header));

        get_widget!(self_.window_builder, gtk::Box, main_layout);
        self.add(&main_layout);

        get_widget!(self_.window_builder, gtk::ScrolledWindow, thread_list_scrolled);
        let thread_list = ThreadList::new(sender.clone());
        thread_list.setup_signals();

        thread_list_scrolled.add(&thread_list.widget);
        thread_list.widget.show_all();
        self_.thread_list.replace(Some(thread_list));


        get_widget!(self_.window_builder, gtk::Box, thread_box);
        let thread_view = ThreadView::new(sender.clone());
        thread_view.setup_signals();

        thread_box.add(&thread_view.widget);
        thread_view.widget.show_all();
        self_.thread_view.replace(Some(thread_view));

        self.resize(800, 480);
    }

    fn setup_signals(&self) {
        let self_ = MainWindowPrivate::from_instance(self);

        get_widget!(self_.window_builder, gtk::Paned, main_header);
        get_widget!(self_.window_builder, gtk::Paned, main_paned);

        let _width_bind = main_paned.bind_property("position", &main_header, "position")
                                    .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                                    .transform_to(move |_binding, value| {
                                        let _offset = 0; //TODO: this offset was trial and error.
                                                        // we should calculate it somehow.
                                        Some((value.get::<i32>().unwrap_or(Some(0)).unwrap()).to_value())
                                    })
                                    .build();

        // window gets closed
        self.connect_delete_event(move |window, _| {
            debug!("Saving window geometry.");
            let _width = window.get_size().0;
            let _height = window.get_size().1;

            // settings_manager::set_integer(Key::WindowWidth, width);
            // settings_manager::set_integer(Key::WindowHeight, height);
            Inhibit(false)
        });
    }

    fn setup_gactions(&self, _sender: Sender<Action>) {
        // We need to upcast from MainWindow to gtk::ApplicationWindow, because MainWindow
        // currently doesn't implement GLib.ActionMap, since it's not supported in gtk-rs for subclassing (13-01-2020)
        let window = self.clone().upcast::<gtk::ApplicationWindow>();
        let _app = window.get_application().unwrap();
    }

    pub fn set_query(&self, query: Arc<notmuch::Query<'static>>) {
        let self_ = MainWindowPrivate::from_instance(self);
        let threads = <notmuch::Query as notmuch::QueryExt>::search_threads(query).unwrap();
        self_.thread_list.borrow().as_ref().unwrap().set_threads(threads);
    }

    pub fn open_thread(&self, thread: Option<Thread>) {

        let self_ = MainWindowPrivate::from_instance(self);

        match thread {
            Some(thread) => {
                self.update_titlebar(Some(&thread.subject()));
                self_.thread_view.borrow().as_ref().unwrap().show_thread(thread)
            },
            None => {
                self.update_titlebar(None);
            }
        }
    }


    pub fn update_titlebar(&self, title: Option<&str>) {
        let self_ = MainWindowPrivate::from_instance(self);
        get_widget!(self_.window_builder, gtk::HeaderBar, conversation_header);
        conversation_header.set_subtitle(title);
    }


}

// #[derive(Msg)]
// pub enum Msg {
//     TagSelect(Option<String>),
//     ThreadSelect(Thread),
//     Change,
//     Quit,
// }

// #[derive(Clone)]
// pub struct Model {
//     relm: Relm<MainWindow>,
//     app: Rc<EnamelApp>
// }

// #[derive(Clone)]
// struct Widgets {
//     headerbar: Component<HeaderBar>,
//     taglist: Component<TagList>,
//     threadlist: Component<ThreadList>,
//     threadview: Component<ThreadView>
// }



// // TODO: Factor out the hamburger menu
// // TODO: Make a proper state machine for the headerbar states
// pub struct MainWindow {
//     model: Model,
//     container: gtk::ApplicationWindow,
//     widgets: Widgets
// }

// impl MainWindow {

//     fn on_tag_changed(self: &mut Self, tag: Option<String>){

//         // TODO: build a new query and refresh the thread list.
//         let dbman = self.model.app.dbmanager.clone();
//         let db = dbman.get(DatabaseMode::ReadOnly).unwrap();

//         let qs = match tag{
//             Some(tag) => format!("tag:{}", tag).to_string(),
//             None => "".to_string()
//         };
//         debug!("qs: {:?}", qs);


//         let query = <notmuch::Database as notmuch::DatabaseExt>::create_query(db, &qs).unwrap();
//         let threads = <notmuch::Query<'_> as notmuch::QueryExt>::search_threads(query).unwrap();

//         self.widgets.threadlist.emit(ThreadListMsg::Update(Some(threads)));
//     }

//     fn on_thread_selected(self: &mut Self, thread: Thread){
//         self.widgets.threadview.emit(ThreadViewMsg::ShowThread(thread))
//     }


// }

// impl Update for MainWindow{
//     type Model = Model;
//     type ModelParam = Rc<EnamelApp>;
//     type Msg = Msg;

//     fn model(relm: &Relm<Self>, app: Self::ModelParam) -> Model {
//         Self::Model {
//             relm: relm.clone(),
//             app
//         }
//     }

//     fn update(&mut self, event: Msg) {
//         match event {
//             Msg::TagSelect(tag) => self.on_tag_changed(tag),
//             Msg::ThreadSelect(thread) => self.on_thread_selected(thread),
//             Msg::Change => {
//                 // self.model.content = self.widgets.input.get_text()
//                 //                                        .expect("get_text failed")
//                 //                                        .chars()
//                 //                                        .rev()
//                 //                                        .collect();
//                 // self.widgets.label.set_text(&self.model.content);
//             },
//             Msg::Quit => gtk::main_quit(),
//         }
//     }
// }

// impl Widget for MainWindow {
//     type Root = gtk::ApplicationWindow;

//     fn root(&self) -> Self::Root {
//         self.container.clone()
//     }

//     fn view(relm: &Relm<Self>, model: Self::Model) -> Self {

//         let window = model.app.builder.get_object::<gtk::ApplicationWindow>("main_window")
//                                   .expect("Couldn't find main_window in ui file.");
//         window.set_application(Some(&model.app.instance));


//         let headerbar = relm_init::<HeaderBar>(model.app.clone()).unwrap();
//         let taglist = relm_init::<TagList>(model.app.clone()).unwrap();
//         let threadlist = relm_init::<ThreadList>(model.app.clone()).unwrap();
//         let threadview = relm_init::<ThreadView>(model.app.clone()).unwrap();


//         // TODO: what would be the best place to connect all UI signals?
//         use self::TagListMsg::ItemSelect as TagList_ItemSelect;
//         connect!(taglist@TagList_ItemSelect(ref tag), relm, Msg::TagSelect(tag.clone()));

//         use self::ThreadListMsg::ThreadSelect as ThreadList_ThreadSelect;
//         connect!(threadlist@ThreadList_ThreadSelect(ref thread), relm, Msg::ThreadSelect(thread.as_ref().unwrap().clone()));



//         MainWindow {
//             model,
//             container: window,
//             widgets: Widgets{
//                 headerbar,
//                 taglist,
//                 threadlist,
//                 threadview
//             }
//         }

//     }

//     fn init_view(&mut self) {

//         let main_paned = self.model.app.builder.get_object::<gtk::Paned>("main_paned")
//                                    .expect("Couldn't find main_paned in ui file.");

//         let threadlist_header = self.model.app.builder.get_object::<gtk::HeaderBar>("threadlist_header")
//                                  .expect("Couldn't find threadlist_header in ui file.");

//         // // TODO: do I need to unbind this at some point?
//         // let _width_bind = main_paned.bind_property("position", &threadlist_header, "width-request")
//         //                             .flags(glib::BindingFlags::SYNC_CREATE)
//         //                             .transform_to(move |_binding, value| {
//         //                                 let offset = 6; //TODO: this offset was trial and error.
//         //                                                 // we should calculate it somehow.
//         //                                 Some((value.get::<i32>().unwrap_or(Some(0)) + offset).to_value())
//         //                             })
//         //                             .build();

//         self.container.show_all();

//         self.widgets.taglist.emit(TagListMsg::Refresh);
//     }

// }