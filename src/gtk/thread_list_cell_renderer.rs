use std::sync::{Once, ONCE_INIT};
use std::cell::Cell;
use std::ptr;
use std::mem;
use std::ffi::CString;

use gio;
use glib;
use gtk;
use gdk;
use cairo;
use glib::translate::*;
use gtk::prelude::*;
use glib_ffi;
use gobject_ffi;
use cairo_ffi;
use gtk_ffi;
use gdk_ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib::IsA;

use notmuch;


use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;


use notmuch::DatabaseMode;

use gobject_subclass::object::*;
use gobject_subclass::properties::*;

use cell_renderer::*;


pub trait CellRendererThreadImpl: 'static {

}

pub struct CellRendererThread {
    thread: Cell<Option<notmuch::Thread>>
}

static PROPERTIES: [Property; 0] = [
    // Property::Boxed(
    //     "thread",
    //     "Thread to display",
    //     "Handle of notmuch::Thread to display",
    //     (1, u32::MAX),
    //     DEFAULT_SAMPLES_PER_BUFFER,
    //     PropertyMutability::ReadWrite,
    // ),


];

impl CellRendererThread {
    pub fn new() -> CellRenderer {
        use glib::object::Downcast;

        static ONCE: Once = ONCE_INIT;
        static mut TYPE: glib::Type = glib::Type::Invalid;

        ONCE.call_once(|| {
            let static_instance = CellRendererThreadStatic::default();
            let t = register_type(static_instance);
            unsafe {
                TYPE = t;
            }
        });

        unsafe {
            glib::Object::new(TYPE, &[]).unwrap().downcast_unchecked()
        }
    }

    fn class_init(klass: &mut CellRendererClass) {
        klass.install_properties(&PROPERTIES);
    }

    fn init(renderer: &CellRenderer) -> Box<CellRendererImpl<CellRenderer>>
    {
        let imp = Self{
            thread: Cell::new(None)
        };
        Box::new(imp)
    }
}

impl ObjectImpl<CellRenderer> for CellRendererThread{


    fn set_property(&self, obj: &glib::Object, id: u32, value: &glib::Value) {
        let prop = &PROPERTIES[id as usize];

        match *prop {
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
        let prop = &PROPERTIES[id as usize];

        match *prop {
            _ => unimplemented!(),
        }
    }
}

impl CellRendererImpl<CellRenderer> for CellRendererThread {

    fn render(&self,
        renderer: &CellRenderer,
        cr: &cairo::Context,
        widget: &gtk::Widget,
        _background_area: &gtk::Rectangle,
        cell_area: &gtk::Rectangle,
        _flags: gtk::CellRendererState,
    ){

        // let layout = widget.create_pango_layout(self.text.borrow().as_str()).unwrap();
        // let sc = widget.get_style_context().unwrap();
        // let (padx, pady) = renderer.get_padding();
        //
        // cr.save();
        // cr.rectangle(cell_area.x.into(), cell_area.y.into(), cell_area.width.into(), cell_area.height.into());
        // cr.clip();
        //
        // gtk::render_layout(&sc, cr, (cell_area.x + padx).into(), (cell_area.y + pady).into(), &layout);
        //
        // cr.restore();
    }

}

#[derive(Default)]
pub struct CellRendererThreadStatic{
}

impl ImplTypeStatic<CellRenderer> for CellRendererThreadStatic {
    fn get_name(&self) -> &str {
        "CellRendererThread"
    }

    fn new(&self, renderer: &CellRenderer) -> Box<CellRendererImpl<CellRenderer>> {
        CellRendererThread::init(renderer)
    }

    fn class_init(&self, klass: &mut CellRendererClass) {
        CellRendererThread::class_init(klass);
    }

    fn type_init(&self, _token: &TypeInitToken, _type: glib::Type) {

    }
}
