//! # Basic test
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

use std::ptr;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Once, ONCE_INIT};

use glib_ffi;
use gobject_ffi;
use gtk_ffi;
use gio_ffi;

use gio;
use glib;
use gtk;
use glib::translate::*;
use gio::prelude::*;
use gtk::prelude::*;

use crossbeam_channel::{unbounded, Receiver, Sender};

use gobject_subclass::object::*;
use gio_subclass::application::{Application as GApplication,
                                ApplicationImpl as GApplicationImpl,
                                ApplicationBase as GApplicationBase};
use gtk_subclass::application::{Application as GtkApplication,
                                ApplicationClass as GtkApplicationClass,
                                GtkApplicationImpl};


mod imp {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct InoxApplication{
        window: gtk::ApplicationWindow,
        overlay: gtk::Overlay,
        settings: gio::Settings,
        content: Rc<Content>,
        headerbar: Rc<Header>,
        player: Rc<player::PlayerWidget>,
        sender: Sender<Action>,
        receiver: Receiver<Action>,
    }

    static PROPERTIES: [Property; 0] = [];

    impl InoxApplication {
        pub fn get_type() -> glib::Type {
            static ONCE: Once = ONCE_INIT;
            static mut TYPE: glib::Type = glib::Type::Invalid;

            ONCE.call_once(|| {
                let static_instance = InoxApplicationStatic;
                let t = register_type(static_instance);
                unsafe {
                    TYPE = t;
                }
            });

            unsafe { TYPE }
        }

        fn class_init(klass: &mut GtkApplicationClass) {
            klass.install_properties(&PROPERTIES);
        }

        fn init(_application: &GtkApplication) -> Box<GtkApplicationImpl<GtkApplication>> {
            let imp = Self {

            };
            Box::new(imp)
        }

    }

    impl ObjectImpl<GtkApplication> for InoxApplication {}

    impl GApplicationImpl<GtkApplication> for InoxApplication
    {
        fn startup(&self, application: &GtkApplication){
            application.parent_startup();

        }
    }

    impl GtkApplicationImpl<GtkApplication> for InoxApplication {}


    pub struct InoxApplicationStatic;

    impl ImplTypeStatic<GtkApplication> for InoxApplicationStatic
    {
        fn get_name(&self) -> &str {
            "InoxApplication"
        }

        fn new(&self, application: &GtkApplication) -> Box<GtkApplicationImpl<GtkApplication>> {
            InoxApplication::init(application)
        }

        fn class_init(&self, klass: &mut GtkApplicationClass) {
            InoxApplication::class_init(klass);
        }
    }
}


glib_wrapper! {
    pub struct InoxApplication(Object<imp::InoxApplication>):
        [GtkApplication => InstanceStruct<GtkApplication>,
         GApplication => InstanceStruct<GApplication>,
         gtk::Application => gtk_ffi::GtkApplication,
         gio::Application => gio_ffi::GApplication,
         gio::ActionGroup => gio_ffi::GActionGroup,
         gio::ActionMap => gio_ffi::GActionMap];

    match fn {
        get_type => || imp::InoxApplication::get_type().to_glib(),
     }
 }


impl InoxApplication {
    pub fn new<'a, I: Into<Option<&'a str>>>(application_id: I, flags: gio::ApplicationFlags) -> Result<InoxApplication, glib::BoolError> {
        use glib::object::Downcast;

        // see gtk-rs/gtk#555
        try!(gtk::init());

        unsafe {
            match glib::Object::new(Self::static_type(), &[("application_id", &application_id.into()),
                                                           ("flags", &flags)]){
                Ok(obj) => Ok(obj.downcast_unchecked()),
                Err(_) => Err(glib::BoolError("Failed to create application"))
            }
        }
    }
}

gobject_subclass_deref!(InoxApplication, GtkApplication);
