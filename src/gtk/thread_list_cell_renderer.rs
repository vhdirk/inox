use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::ptr;
use std::mem;

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

// A lot of the stuff below was generated with an adapted version of gobject_gen
// (https://gitlab.gnome.org/federico/gnome-class), but I could not get the macroto work reliably
// with each nightly compiler. Pasting it here is rather verbose but it does make sure it will
// work consistently


#[derive(Default)]
struct CellRendererThreadPriv {}

glib_wrapper ! {
    pub struct CellRendererThread(Object<CellRendererThreadFfi, CellRendererThreadClass>)
    : [gtk::CellRenderer => gtk_ffi::GtkCellRenderer];


    match fn {
        get_type => || cell_renderer_thread_get_type(),
    }
}

pub struct CellRendererThreadFfi {
    pub parent: <gtk::CellRenderer as glib::wrapper::Wrapper>::GlibType,
}

#[repr(C)]
pub struct CellRendererThreadClass {
    pub parent_class: <gtk::CellRenderer as glib::wrapper::Wrapper>::GlibClassType,
}

struct CellRendererThreadClassPrivate {
    parent_class: *const <gtk::CellRenderer as glib::wrapper::Wrapper>::GlibClassType,
}

static mut PRIV: CellRendererThreadClassPrivate = CellRendererThreadClassPrivate {
    parent_class: 0 as *const _,
};

impl CellRendererThread {

    pub fn new() -> CellRendererThread {
        unsafe { from_glib_full(cell_renderer_thread_new()) }
    }

    #[allow(dead_code)]
    fn get_priv(&self) -> &CellRendererThreadPriv {
        unsafe {
            let _private = gobject_ffi::g_type_instance_get_private(
                <Self as ToGlibPtr<*mut CellRendererThreadFfi>>::to_glib_none(self).0
                    as *mut gobject_ffi::GTypeInstance,
                cell_renderer_thread_get_type(),
            ) as *const Option<CellRendererThreadPriv>;
            (&*_private).as_ref().unwrap()
        }
    }

    fn render_impl(
        &self,
        cr: &cairo::Context,
        widget: &gtk::Widget,
        background_area: &gtk::Rectangle,
        cell_area: &gtk::Rectangle,
        flags: gtk::CellRendererState,
    ){

    }


}
impl CellRendererThreadFfi {
    #[allow(dead_code)]
    fn get_class(&self) -> &CellRendererThreadClass {
        unsafe {
            let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
            &*(klass as *const CellRendererThreadClass)
        }
    }
    unsafe extern "C" fn init(
        obj: *mut gobject_ffi::GTypeInstance,
        _klass: glib_ffi::gpointer,
    ) {
        #[allow(unused_variables)]
        let obj = obj;
        #[allow(deprecated)]
        let _guard = glib::CallbackGuard::new();
        let _private =
            gobject_ffi::g_type_instance_get_private(obj, cell_renderer_thread_get_type())
                as *mut Option<CellRendererThreadPriv>;
        ptr::write(
            _private,
            Some(<CellRendererThreadPriv as Default>::default()),
        );
    }
    unsafe extern "C" fn finalize(obj: *mut gobject_ffi::GObject) {
        #[allow(deprecated)]
        let _guard = glib::CallbackGuard::new();
        let _private = gobject_ffi::g_type_instance_get_private(
            obj as *mut gobject_ffi::GTypeInstance,
            cell_renderer_thread_get_type(),
        ) as *mut Option<CellRendererThreadPriv>;
        let _ = (*_private).take();
        (*(PRIV.parent_class as *mut gobject_ffi::GObjectClass))
            .finalize
            .map(|f| f(obj));
    }

    unsafe extern "C" fn render_slot_trampoline(
        this: *mut <gtk::CellRenderer as glib::wrapper::Wrapper>::GlibType,
        cr: *mut cairo_ffi::cairo_t,
        widget: *mut gtk_ffi::GtkWidget,
        background_area: *const gdk_ffi::GdkRectangle,
        cell_area: *const gdk_ffi::GdkRectangle,
        flags: gtk_ffi::GtkCellRendererState,
    )  {
        #[allow(deprecated)]
        let _guard = glib::CallbackGuard::new();
        let this = this as *mut CellRendererThreadFfi;
        let instance: &CellRendererThread = &from_glib_borrow(this);
        instance.render_impl(
            &<cairo::Context as FromGlibPtrBorrow<_>>::from_glib_borrow(cr),
            &<gtk::Widget as FromGlibPtrBorrow<_>>::from_glib_borrow(widget),
            &<gdk::Rectangle as FromGlibPtrBorrow<_>>::from_glib_borrow(background_area),
            &<gdk::Rectangle as FromGlibPtrBorrow<_>>::from_glib_borrow(cell_area),
            glib::translate::FromGlib::from_glib(flags)
        )
    }
}

impl CellRendererThreadClass {
    unsafe extern "C" fn init(klass: glib_ffi::gpointer, _klass_data: glib_ffi::gpointer) {
        #[allow(deprecated)]
        let _guard = glib::CallbackGuard::new();
        gobject_ffi::g_type_class_add_private(
            klass,
            mem::size_of::<Option<CellRendererThreadPriv>>(),
        );
        {
            let gobject_class = &mut *(klass as *mut gobject_ffi::GObjectClass);
            gobject_class.finalize = Some(CellRendererThreadFfi::finalize);
        }
        {
            #[allow(unused_variables)]
            let klass = &mut *(klass as *mut CellRendererThreadClass);

            (*(klass as *mut _ as *mut <gtk::CellRenderer as glib::wrapper::Wrapper>::GlibClassType))
                .render = Some(CellRendererThreadFfi::render_slot_trampoline);
        }
        {}
        PRIV.parent_class = gobject_ffi::g_type_class_peek_parent(klass)
            as *const <gtk::CellRenderer as glib::wrapper::Wrapper>::GlibClassType;
    }
}


#[no_mangle]
pub unsafe extern "C" fn cell_renderer_thread_new() -> *mut CellRendererThreadFfi {
    #[allow(deprecated)]
    let _guard = glib::CallbackGuard::new();
    let this =
        gobject_ffi::g_object_newv(cell_renderer_thread_get_type(), 0, ptr::null_mut());
    this as *mut CellRendererThreadFfi
}


#[no_mangle]
pub unsafe extern "C" fn cell_renderer_thread_get_type() -> glib_ffi::GType {
    #[allow(deprecated)]
    let _guard = glib::CallbackGuard::new();
    use std::sync::{Once, ONCE_INIT};
    use std::u16;
    static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
    static ONCE: Once = ONCE_INIT;
    ONCE.call_once(|| {
        let class_size = mem::size_of::<CellRendererThreadClass>();
        assert!(class_size <= u16::MAX as usize);
        let instance_size = mem::size_of::<CellRendererThreadFfi>();
        assert!(instance_size <= u16::MAX as usize);
        TYPE = gobject_ffi::g_type_register_static_simple(
            <gtk::CellRenderer as glib::StaticType>::static_type().to_glib(),
            b"CellRendererThread\x00" as *const u8 as *const i8,
            class_size as u32,
            Some(CellRendererThreadClass::init),
            instance_size as u32,
            Some(CellRendererThreadFfi::init),
            gobject_ffi::GTypeFlags::empty(),
        );
    });
    TYPE
}


pub trait CellRendererThreadExt {}

impl<O: IsA<CellRendererThread> + IsA<glib::object::Object> + glib::object::ObjectExt>
    CellRendererThreadExt for O
{



}
