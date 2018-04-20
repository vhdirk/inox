use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::ptr;
use std::mem;

use gio;
use glib;
use gtk;
use glib::IsA;
use glib::translate::*;
use gtk::prelude::*;
use gobject_gen::gobject_gen;
use relm_attributes::widget;

use notmuch;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;

use notmuch::DatabaseMode;


struct CellRendererThreadPrivate{

}

gobject_gen! {
    class CellRendererThreadClass: gtk::CellRenderer {
        type InstancePrivate = CellRendererThreadPrivate;
    }
}
