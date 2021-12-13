use std::cell::RefCell;
use gtk::subclass::widget::WidgetImplExt;
use glib::{self, prelude::*, subclass::prelude::*};
use gtk::subclass::prelude::*;


pub type GizmoInstance = super::Gizmo;

pub type GizmoMeasureFunc = fn(&GizmoInstance, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32);
pub type GizmoSizeAllocateFunc = fn(&GizmoInstance, width: i32, height: i32, baseline: i32);
pub type GizmoSnapshotFunc = fn(&GizmoInstance, snapshot: &gtk::Snapshot);
pub type GizmoContainsFunc = fn(&GizmoInstance, x: f64, y: f64) -> bool;
pub type GizmoFocusFunc = fn(&GizmoInstance, direction: gtk::DirectionType) -> bool;
pub type GizmoGrabFocusFunc = fn(&GizmoInstance) -> bool;

pub struct Gizmo {
    pub measure: RefCell<Option<GizmoMeasureFunc>>,
    pub size_allocate: RefCell<Option<GizmoSizeAllocateFunc>>,
    pub snapshot: RefCell<Option<GizmoSnapshotFunc>>,
    pub contains: RefCell<Option<GizmoContainsFunc>>,
    pub focus: RefCell<Option<GizmoFocusFunc>>,
    pub grab_focus: RefCell<Option<GizmoGrabFocusFunc>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Gizmo {
    const NAME: &'static str = "InoxGizmo";
    type Type = GizmoInstance;
    type ParentType = gtk::Widget;

    fn new() -> Self {
        Self {
            measure: RefCell::new(None::<GizmoMeasureFunc>),
            size_allocate: RefCell::new(None),
            snapshot: RefCell::new(None),
            contains: RefCell::new(None),
            focus: RefCell::new(None),
            grab_focus: RefCell::new(None),
        }
    }
}

impl ObjectImpl for Gizmo {}

impl WidgetImpl for Gizmo {
    fn measure(
        &self,
        widget: &Self::Type,
        orientation: gtk::Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        if let Some(func) = self.measure.borrow().as_ref() {
            func(widget, orientation, for_size)
        } else {
            self.parent_measure(widget, orientation, for_size)
        }
    }

    fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
        if let Some(func) = self.size_allocate.borrow().as_ref() {
            func(widget, width, height, baseline)
        } else {
            self.parent_size_allocate(widget, width, height, baseline)
        }
    }

    fn snapshot(&self, widget: &Self::Type, snapshot: &gtk::Snapshot) {
        if let Some(func) = self.snapshot.borrow().as_ref() {
            func(widget, snapshot)
        } else {
            self.parent_snapshot(widget, snapshot)
        }
    }

    fn contains(&self, widget: &Self::Type, x: f64, y: f64) -> bool {
        if let Some(func) = self.contains.borrow().as_ref() {
            func(widget, x, y)
        } else {
            self.parent_contains(widget, x, y)
        }
    }

    fn focus(&self, widget: &Self::Type, direction_type: gtk::DirectionType) -> bool {
        if let Some(func) = self.focus.borrow().as_ref() {
            func(widget, direction_type)
        } else {
            self.parent_focus(widget, direction_type)
        }
    }

    fn grab_focus(&self, widget: &Self::Type) -> bool {
        if let Some(func) = self.grab_focus.borrow().as_ref() {
            func(widget)
        } else {
            self.parent_grab_focus(widget)
        }
    }
}
