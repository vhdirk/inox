use std::cell::RefCell;
use std::sync::Arc;

use adw;
use adw::prelude::*;
use futures::future::FutureExt;
use gio::prelude::*;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::traits::{GtkWindowExt, WidgetExt};
use gtk::{prelude::*, CompositeTemplate};
use log::*;

use crate::app::{Action, InoxApplication};
use crate::main_header::MainHeader;

// use crate::headerbar::HeaderBar;
use inox_core::database::Thread;

use crate::widgets::thread_list::ThreadList;
use crate::widgets::thread_view::ThreadView;

mod imp {
    use adw::subclass::prelude::{AdwApplicationWindowImpl, *};
    use gtk::subclass::prelude::*;
    use gtk::{prelude::*, subclass::prelude::*, CompositeTemplate};
    use once_cell::unsync::OnceCell;
    use std::cell::RefCell;

    use crate::main_header::MainHeader;
    use crate::widgets::thread_list::ThreadList;
    use crate::widgets::thread_view::ThreadView;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/github/vhdirk/Inox/gtk/main_window.ui")]
    pub struct MainWindow {
        // #[template_child]
        // pub main_header: TemplateChild<gtk::HeaderBar>,

        // #[template_child]
        // pub main_layout: TemplateChild<gtk::Box>,

        // #[template_child]
        // pub main_paned: TemplateChild<gtk::Paned>,
        #[template_child]
        pub thread_list_box: TemplateChild<gtk::Box>,

        // menu_builder: gtk::Builder,
        pub thread_list: OnceCell<ThreadList>,

        #[template_child]
        pub thread_view_box: TemplateChild<gtk::Box>,

        pub thread_view: OnceCell<ThreadView>,
        // thread_view: RefCell<Option<ThreadView>>, // current_notification: RefCell<Option<Rc<Notification>>>,
    }

    impl Default for MainWindow {
        fn default() -> Self {
            MainWindow {
                // main_header: TemplateChild::default(),
                // main_layout: TemplateChild::default(),
                // main_paned: TemplateChild::default(),
                thread_list_box: TemplateChild::default(),
                thread_list: OnceCell::new(),

                thread_view_box: TemplateChild::default(),
                thread_view: OnceCell::new(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "InoxMainWindow";
        type Type = super::MainWindow;
        type ParentType = adw::ApplicationWindow;

        // Within class_init() you must set the template.
        // The CompositeTemplate derive macro provides a convenience function
        // bind_template() to set the template and bind all children at once.
        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Implement GLib.Object for MainWindow
    impl ObjectImpl for MainWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.setup_all();
        }
    }

    impl WidgetImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl ApplicationWindowImpl for MainWindow {}
    impl AdwApplicationWindowImpl for MainWindow {}
}

// Wrap imp::MainWindow into a usable gtk-rs object
glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

// MainWindow implementation itself
impl MainWindow {
    pub fn new(sender: Sender<Action>, app: InoxApplication) -> Self {
        // Create new GObject and downcast it into MainWindow
        let window: Self = glib::Object::new(&[("application", &app)])
            .expect("Failed to create ApplicationWindow");

        app.add_window(&window.clone());
        window.setup_widgets(sender.clone());
        // // window.setup_signals();
        // // window.setup_gactions(sender);
        window
    }

    pub fn setup_all(&self) {
        let imp = imp::MainWindow::from_instance(self);
    }

    pub fn setup_widgets(&self, sender: Sender<Action>) {
        let mut imp = imp::MainWindow::from_instance(self);
        let app: InoxApplication = self
            .application()
            .unwrap()
            .downcast::<InoxApplication>()
            .unwrap();

        // Add headerbar/content to the window itself
        //self.set_titlebar(Some(&imp.main_header.get()));

        // get_widget!(imp.window_builder, gtk::Box, main_layout);
        // self.set_child(Some(&imp.main_layout.get()));

        let thread_list = ThreadList::new(sender.clone());
        thread_list.set_parent(&imp.thread_list_box.get());
        thread_list.show();
        imp.thread_list_box.show();
        imp.thread_list
        .set(thread_list)
        .expect("Thread list box was not empty");
        // // thread_list.setup_signals();

        let thread_view = ThreadView::new(sender.clone());
        thread_view.set_parent(&imp.thread_view_box.get());
        thread_view.show();
        imp.thread_view_box.show();
        imp.thread_view
            .set(thread_view)
            .expect("Thread view box was not empty");

        // thread_view.setup_signals();

        // thread_box.add(&thread_view.widget);
        // thread_view.widget.show_all();
        // imp.thread_view.replace(Some(thread_view));

        // self.resize(800, 480);
    }

    // fn setup_signals(&self) {
    //     let imp = imp::MainWindow::from_instance(self);

    //     // get_widget!(imp.window_builder, gtk::Paned, main_header);
    //     // get_widget!(imp.window_builder, gtk::Paned, main_paned);

    //     let _width_bind = imp
    //         .main_paned
    //         .get()
    //         .bind_property("position", &imp.main_header.get(), "position")
    //         .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
    //         .transform_to(move |_binding, value| {
    //             let _offset = 0; //TODO: this offset was trial and error.
    //                              // we should calculate it somehow.
    //             Some((value.get::<i32>().unwrap_or(0)).to_value())
    //         })
    //         .build();

    //     // // window gets closed
    //     // self.connect_delete_event(move |window, _| {
    //     //     debug!("Saving window geometry.");
    //     //     // let _width = window.size().0;
    //     //     // let _height = window.size().1;

    //     //     // settings_manager::set_integer(Key::WindowWidth, width);
    //     //     // settings_manager::set_integer(Key::WindowHeight, height);
    //     //     gtk::Inhibit(false)
    //     // });
    // }

    // // fn setup_gactions(&self, _sender: Sender<Action>) {
    // //     // We need to upcast from MainWindow to gtk::ApplicationWindow, because MainWindow
    // //     // currently doesn't implement GLib.ActionMap, since it's not supported in gtk-rs for subclassing (13-01-2020)
    // //     let window = self.clone().upcast::<gtk::ApplicationWindow>();
    // //     let _app = window.application().unwrap();
    // // }

    pub fn set_query(&self, query: &notmuch::Query) {
        let imp = imp::MainWindow::from_instance(self);
        let threads = query.search_threads().unwrap();
        imp.thread_list.get().unwrap().set_threads(threads);
    }

    pub fn open_thread(&self, thread: Option<Thread>) {
        let imp = imp::MainWindow::from_instance(self);

        match thread {
            Some(thread) => {
                // self.update_titlebar(Some(&thread.subject()));

                let thread_view = imp.thread_view.get().unwrap();
                thread_view.load_thread(thread);
            }
            None => {
                // self.update_titlebar(None);
            }
        }
    }

    // pub fn update_titlebar(&self, title: Option<&str>) {
    //     let imp = imp::MainWindow::from_instance(self);
    //     get_widget!(imp.window_builder, gtk::HeaderBar, conversation_header);
    //     conversation_header.set_subtitle(title);
    // }
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
//     app: Rc<InoxApp>
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
//     type ModelParam = Rc<InoxApp>;
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
