use gio::prelude::*;
use glib::clone;
use glib::subclass::prelude::*;
use glib::Sender;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::SingleSelection;
use gtk::{Application, SignalListItemFactory};
use log::*;
use std::fmt;

use crate::widgets::resize_leaflet::gizmo_imp::GizmoContainsFunc;
use crate::widgets::resize_leaflet::gizmo_imp::GizmoFocusFunc;
use crate::widgets::resize_leaflet::gizmo_imp::GizmoGrabFocusFunc;
use crate::widgets::resize_leaflet::gizmo_imp::GizmoMeasureFunc;
use crate::widgets::resize_leaflet::gizmo_imp::GizmoSizeAllocateFunc;
use crate::widgets::resize_leaflet::gizmo_imp::GizmoSnapshotFunc;

use super::gizmo_imp as imp;

glib::wrapper! {
    pub struct Gizmo(ObjectSubclass<imp::Gizmo>) @extends gtk::Widget;
}

// Gizmo implementation itself
impl Gizmo {
    pub fn new(
        css_name: &str,
        measure: Option<GizmoMeasureFunc>,
        size_allocate: Option<GizmoSizeAllocateFunc>,
        snapshot: Option<GizmoSnapshotFunc>,
        contains: Option<GizmoContainsFunc>,
        focus: Option<GizmoFocusFunc>,
        grab_focus: Option<GizmoGrabFocusFunc>,
    ) -> Self {
        glib::Object::new(&[("css-name", &css_name.to_string())]).expect("Failed to create Gizmo")
    }
}
