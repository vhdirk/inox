use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::ptr;
use std::mem;

use gio;
use glib;
use glib::IsA;
use glib::translate::*;
use gtk_ffi;
use gtk::prelude::*;
use relm_attributes::widget;

use notmuch;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;

use notmuch::DatabaseMode;

gobject_gen! {
    class CellRendererThread: gtk::CellRenderer {
    }
}
