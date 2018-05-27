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

use gobject_subclass::object::*;
use gio_subclass::application::{Application as GApplication, ApplicationImpl, ApplicationBase};
use gtk_subclass::application::{Application as GtkApplication};
use gtk_subclass::application::*;


mod imp {
    use super::*;

    pub struct InoxApplication{

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

        fn class_init(klass: &mut ApplicationClass) {
            klass.install_properties(&PROPERTIES);
        }

        fn init(_application: &Application) -> Box<GtkApplicationImpl<Application>> {
            let imp = Self {

            };
            Box::new(imp)
        }

    }

    impl ObjectImpl<Application> for InoxApplication {}

    impl ApplicationImpl<Application> for InoxApplication
    {
        fn startup(&self, application: &Application){
            application.parent_startup();

        }
    }

    impl GtkApplicationImpl<Application> for InoxApplication {}


    pub struct InoxApplicationStatic;

    impl ImplTypeStatic<Application> for InoxApplicationStatic
    {
        fn get_name(&self) -> &str {
            "InoxApplication"
        }

        fn new(&self, application: &Application) -> Box<GtkApplicationImpl<Application>> {
            InoxApplication::init(application)
        }

        fn class_init(&self, klass: &mut ApplicationClass) {
            InoxApplication::class_init(klass);
        }
    }
}


glib_wrapper! {
    pub struct InoxApplication(Object<imp::InoxApplication>):
        [Application => InstanceStruct<Application>,
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

// TODO: This one should probably get a macro
impl Deref for InoxApplication {
    type Target = imp::InoxApplication;

    fn deref(&self) -> &Self::Target {
        unsafe {

            let base: Application = from_glib_borrow(self.to_glib_none().0);
            let imp = base.get_impl();
            let imp = imp.downcast_ref::<imp::InoxApplication>().unwrap();
            // Cast to a raw pointer to get us an appropriate lifetime: the compiler
            // can't know that the lifetime of base is the same as the one of self
            &*(imp as *const imp::InoxApplication)
        }
    }
}
