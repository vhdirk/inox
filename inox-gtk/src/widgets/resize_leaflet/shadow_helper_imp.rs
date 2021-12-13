use adw;
use glib::subclass::prelude::*;
use graphene;
use gsk;
use gtk;
use gtk::prelude::*;
use std::cell::RefCell;

use glib::subclass::prelude::*;
use glib::subclass::signal::Signal;
use glib::{ParamFlags, ParamSpec, ParamSpecObject, Value};
use gtk::{prelude::*, subclass::prelude::*};
use once_cell::sync::Lazy;
use once_cell::unsync::OnceCell;
use std::cmp;

use super::Gizmo;

#[derive(Debug, Default)]
pub struct ShadowHelper {
    widget: RefCell<Option<Gizmo>>,
    dimming: OnceCell<Gizmo>,
    shadow: OnceCell<Gizmo>,
    border: OnceCell<Gizmo>,
    outline: OnceCell<Gizmo>,
}

#[glib::object_subclass]
impl ObjectSubclass for ShadowHelper {
    const NAME: &'static str = "InoxShadowHelper";
    type Type = super::ShadowHelper;
    type ParentType = glib::Object;

    fn class_init(klass: &mut Self::Class) {}
}

impl ObjectImpl for ShadowHelper {
    fn constructed(&self, obj: &Self::Type) {
        let dimming = Gizmo::new("dimming", None, None, None, None, None, None);
        let shadow = Gizmo::new("shadow", None, None, None, None, None, None);
        let border = Gizmo::new("border", None, None, None, None, None, None);
        let outline = Gizmo::new("outline", None, None, None, None, None, None);

        dimming.set_child_visible(false);
        shadow.set_child_visible(false);
        border.set_child_visible(false);
        outline.set_child_visible(false);

        dimming.set_can_target(false);
        shadow.set_can_target(false);
        border.set_can_target(false);
        outline.set_can_target(false);

        dimming.set_parent(self.widget.borrow().as_ref().unwrap());
        shadow.set_parent(self.widget.borrow().as_ref().unwrap());
        border.set_parent(self.widget.borrow().as_ref().unwrap());
        outline.set_parent(self.widget.borrow().as_ref().unwrap());

        self.dimming.set(dimming).expect("failed to set dimming");
        self.shadow.set(shadow).expect("failed to set shadow");
        self.border.set(border).expect("failed to set border");
        self.outline.set(outline).expect("failed to set outline");

        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {
        if let Some(w) = self.dimming.get() {
            w.unparent();
        }
        if let Some(w) = self.shadow.get() {
            w.unparent();
        }
        if let Some(w) = self.border.get() {
            w.unparent();
        }
        if let Some(w) = self.outline.get() {
            w.unparent();
        }
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                // Name
                "widget",
                // Nickname
                "widget",
                // Short description
                "The widget we're helping",
                // Default value
                gtk::Widget::static_type(),
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "widget" => {
                self.widget.replace(Some(value.get().unwrap()));
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "widget" => self.widget.borrow().as_ref().unwrap().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl ShadowHelper {
    pub fn set_style_classes(&self, direction: gtk::PanDirection) {
        let direction_classes = vec![match direction {
            gtk::PanDirection::Left => "left",
            gtk::PanDirection::Right => "right",
            gtk::PanDirection::Up => "up",
            gtk::PanDirection::Down => "down",
            _ =>  "", // TODO
        }];

        self.dimming
            .get()
            .unwrap()
            .set_css_classes(&direction_classes);
        self.shadow
            .get()
            .unwrap()
            .set_css_classes(&direction_classes);
        self.border
            .get()
            .unwrap()
            .set_css_classes(&direction_classes);
        self.outline
            .get()
            .unwrap()
            .set_css_classes(&direction_classes);
    }

    pub fn size_allocate(
        &self,
        allocation: &gtk::Allocation,
        baseline: i32,
        progress: f64,
        direction: gtk::PanDirection,
    ) {
        self.set_style_classes(direction);

        let x = allocation.x();
        let y = allocation.y();
        let width = allocation.width();
        let height = allocation.height();

        self.dimming.get().unwrap().allocate(
            width,
            height,
            baseline,
            gsk::Transform::new()
                .translate(&graphene::Point::new(x as f32, y as f32))
                .as_ref(),
        );

        let (distance, orientation) = match direction {
            gtk::PanDirection::Left | gtk::PanDirection::Right => {
                (width, gtk::Orientation::Horizontal)
            }
            gtk::PanDirection::Up | gtk::PanDirection::Down => {
                (height, gtk::Orientation::Vertical)
            }
            _ => (0, gtk::Orientation::__Unknown(0)),
        };

        self.dimming
            .get()
            .unwrap()
            .set_child_visible(progress < 1.0);
        self.shadow.get().unwrap().set_child_visible(progress < 1.0);
        self.border.get().unwrap().set_child_visible(progress < 1.0);
        self.outline
            .get()
            .unwrap()
            .set_child_visible(progress < 1.0);

        let (shadow_size, _, _, _) = self.shadow.get().unwrap().measure(orientation, -1);
        let (border_size, _, _, _) = self.border.get().unwrap().measure(orientation, -1);
        let (outline_size, _, _, _) = self.outline.get().unwrap().measure(orientation, -1);

        let remaining_distance = (1.0 - progress) * distance as f64;
        let shadow_opacity = if remaining_distance < shadow_size.into() {
            remaining_distance / shadow_size as f64
        } else {
            1.0
        };

        self.dimming.get().unwrap().set_opacity(1.0 - progress);
        self.shadow.get().unwrap().set_opacity(shadow_opacity);

        match direction {
            gtk::PanDirection::Left => {
                self.shadow.get().unwrap().allocate(
                    shadow_size,
                    cmp::max(height, shadow_size),
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new(x as f32, y as f32))
                        .as_ref(),
                );
                self.border.get().unwrap().allocate(
                    border_size,
                    cmp::max(height, border_size),
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new(x as f32, y as f32))
                        .as_ref(),
                );
                self.outline.get().unwrap().allocate(
                    outline_size,
                    cmp::max(height, outline_size),
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new((x - outline_size) as f32, y as f32))
                        .as_ref(),
                );
            }
            gtk::PanDirection::Right => {
                self.shadow.get().unwrap().allocate(
                    shadow_size,
                    cmp::max(height, shadow_size),
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new((x + width - shadow_size) as f32, y as f32))
                        .as_ref(),
                );
                self.border.get().unwrap().allocate(
                    border_size,
                    cmp::max(height, border_size),
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new((x + width - border_size) as f32, y as f32))
                        .as_ref(),
                );
                self.outline.get().unwrap().allocate(
                    outline_size,
                    cmp::max(height, outline_size),
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new((x + width) as f32, y as f32))
                        .as_ref(),
                );
            }
            gtk::PanDirection::Up => {
                self.shadow.get().unwrap().allocate(
                    cmp::max(width, shadow_size),
                    shadow_size,
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new(x as f32, y as f32))
                        .as_ref(),
                );
                self.border.get().unwrap().allocate(
                    cmp::max(width, border_size),
                    border_size,
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new(x as f32, y as f32))
                        .as_ref(),
                );
                self.outline.get().unwrap().allocate(
                    cmp::max(width, outline_size),
                    outline_size,
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new(x as f32, (y - outline_size) as f32))
                        .as_ref(),
                );
            }
            gtk::PanDirection::Down => {
                self.shadow.get().unwrap().allocate(
                    cmp::max(width, shadow_size),
                    shadow_size,
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new(x as f32, (y + height - shadow_size) as f32))
                        .as_ref(),
                );
                self.border.get().unwrap().allocate(
                    cmp::max(width, border_size),
                    border_size,
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new(x as f32, (y + height - border_size) as f32))
                        .as_ref(),
                );
                self.border.get().unwrap().allocate(
                    cmp::max(width, outline_size),
                    outline_size,
                    baseline,
                    gsk::Transform::new()
                        .translate(&graphene::Point::new(x as f32, (y + height) as f32))
                        .as_ref(),
                );
            },
            _ => { /* Do nothing */ },
        };
    }

    pub fn snapshot(&self, snapshot: &gtk::Snapshot) {
        if !self.dimming.get().unwrap().is_child_visible() {
            return;
        }

        if let Some(widget) = self.widget.borrow().as_ref() {
            widget.snapshot_child(self.dimming.get().unwrap(), snapshot);
            widget.snapshot_child(self.shadow.get().unwrap(), snapshot);
            widget.snapshot_child(self.border.get().unwrap(), snapshot);
            widget.snapshot_child(self.outline.get().unwrap(), snapshot);
        }
    }
}
