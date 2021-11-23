use crate::core::Action;
use glib::IsA;
use glib::Sender;

use glib::{self, prelude::*, subclass::prelude::*};
use gtk::{self, subclass::prelude::*};

use super::base_row_imp as imp;

glib::wrapper! {
    pub struct BaseRow(ObjectSubclass<imp::BaseRow>)
    @extends gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

pub trait BaseRowExt {
    fn expand(&self);
    fn collapse(&self);
}

impl<O: IsA<BaseRow>> BaseRowExt for O {
    fn expand(&self) {
        unsafe { imp::base_row_expand(self.upcast_ref::<BaseRow>()) }
    }

    fn collapse(&self) {
        unsafe { imp::base_row_collapse(self.upcast_ref::<BaseRow>()) }
    }
}

pub trait BaseRowImpl: ListBoxRowImpl + ObjectImpl + 'static {
    fn expand(&self, obj: &BaseRow) {
        self.parent_expand(obj)
    }

    fn collapse(&self, obj: &BaseRow) {
        self.parent_collapse(obj)
    }
}

pub trait BaseRowImplExt: ObjectSubclass {
    fn parent_expand(&self, obj: &BaseRow);
    fn parent_collapse(&self, obj: &BaseRow);
}

impl<T: BaseRowImpl> BaseRowImplExt for T {
    fn parent_expand(&self, obj: &BaseRow) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut imp::BaseRowClass;
            if let Some(ref f) = (*parent_class).expand {
                f(obj)
            } else {
                unimplemented!()
            }
        }
    }

    fn parent_collapse(&self, obj: &BaseRow) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut imp::BaseRowClass;
            if let Some(ref f) = (*parent_class).collapse {
                f(obj)
            } else {
                unimplemented!()
            }
        }
    }
}

/// Make the BaseRow subclassable
unsafe impl<T: BaseRowImpl> IsSubclassable<T> for BaseRow {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class.upcast_ref_mut());

        let klass = class.as_mut();
        klass.expand = Some(expand_trampoline::<T>);
        klass.collapse = Some(collapse_trampoline::<T>);
    }
}

// Virtual method implementation trampolines
unsafe fn expand_trampoline<T>(this: &BaseRow)
where
    T: ObjectSubclass + BaseRowImpl,
{
    let instance = &*(this as *const _ as *const T::Instance);
    let imp = instance.impl_();
    imp.expand(this)
}

unsafe fn collapse_trampoline<T>(this: &BaseRow)
where
    T: ObjectSubclass + BaseRowImpl,
{
    let instance = &*(this as *const _ as *const T::Instance);
    let imp = instance.impl_();
    imp.collapse(this)
}

// // Base class for list rows in the list box
//     internal abstract class ConversationRow : Gtk.ListBoxRow, Geary.BaseInterface {

//         protected const string EXPANDED_CLASS = "geary-expanded";

//         // The email being displayed by this row, if any
//         public Geary.Email? email { get; private set; default = null; }

//         // Is the row showing the email's message body or just headers?
//         public bool is_expanded {
//             get {
//                 return this._is_expanded;
//             }
//             protected set {
//                 this._is_expanded = value;
//                 notify_property("is-expanded");
//             }
//         }
//         private bool _is_expanded = false;

//         // We can only scroll to a specific row once it has been
//         // allocated space. This signal allows the viewer to hook up
//         // to appropriate times to try to do that scroll.
//         public signal void should_scroll();

//         // Emitted when an email is loaded for the first time
//         public signal void email_loaded(Geary.Email email);

//         protected ConversationRow(Geary.Email? email) {
//             base_ref();
//             this.email = email;
//             notify["is-expanded"].connect(update_css_class);
//             show();
//         }

//         ~ConversationRow() {
//             base_unref();
//         }

//         // Request the row be expanded, if supported.
//         public virtual new async void expand()
//             throws GLib.Error {
//             // Not supported by default
//         }

//         // Request the row be collapsed, if supported.
//         public virtual void collapse() {
//             // Not supported by default
//         }

//         // Enables firing the should_scroll signal when this row is
//         // allocated a size
//         public void enable_should_scroll() {
//             this.size_allocate.connect(on_size_allocate);
//         }

//         private void update_css_class() {
//             if (this.is_expanded)
//                 get_style_context().add_class(EXPANDED_CLASS);
//             else
//                 get_style_context().remove_class(EXPANDED_CLASS);

//             update_previous_sibling_css_class();
//         }

//         // This is mostly taken form libhandy HdyExpanderRow
//         private Gtk.Widget? get_previous_sibling() {
//             if (this.parent is Gtk.Container) {
//                 var siblings = this.parent.get_children();
//                 unowned List<weak Gtk.Widget> l;
//                 for (l = siblings; l != null && l.next != null && l.next.data != this; l = l.next);

//                 if (l != null && l.next != null && l.next.data == this) {
//                     return l.data;
//                 }
//             }

//             return null;
//         }

//         private void update_previous_sibling_css_class() {
//             var previous_sibling = get_previous_sibling();
//             if (previous_sibling != null) {
//                 if (this.is_expanded)
//                     previous_sibling.get_style_context().add_class("geary-expanded-previous-sibling");
//                 else
//                     previous_sibling.get_style_context().remove_class("geary-expanded-previous-sibling");
//             }
//         }

//         protected inline void set_style_context_class(string class_name, bool value) {
//             if (value) {
//                 get_style_context().add_class(class_name);
//             } else {
//                 get_style_context().remove_class(class_name);
//             }
//         }

//         protected void on_size_allocate() {
//             // Disable should_scroll so we don't keep on scrolling
//             // later, like when the window has been resized.
//             this.size_allocate.disconnect(on_size_allocate);
//             should_scroll();
//         }

//     }
