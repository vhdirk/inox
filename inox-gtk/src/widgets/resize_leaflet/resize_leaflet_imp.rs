use adw::{self, prelude::*, subclass::prelude::*};
use gdk;
use glib::object::InterfaceRef;
use glib::subclass::signal::Signal;
use glib::Interface;
use glib::{self, prelude::*, subclass::prelude::*};
use glib::{clone, Sender};
use glib::{
    ParamFlags, ParamSpec, ParamSpecBoolean, ParamSpecBoxed, ParamSpecEnum, ParamSpecObject,
    ParamSpecOverride, ParamSpecString, ParamSpecUInt, Value,
};
use gtk::builders::ImageBuilder;
use gtk::SignalListItemFactory;
use gtk::{self, prelude::*, subclass::prelude::*};
use log::*;
use once_cell::sync::{Lazy, OnceCell};
use std::cell::RefCell;
use std::cmp;
use std::fmt;

use crate::widgets::resize_leaflet::ResizeLeafletPage;

use super::ShadowHelper;

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return a * (1.0 - t) + b * t;
}

#[derive(Debug)]
pub struct ModeTransition {
    pub duration: i32,
    pub current_pos: f64,
    pub start_progress: f64,
    pub end_progress: f64,
    pub animation: Option<adw::Animation>,
}

impl Default for ModeTransition {
    fn default() -> Self {
        Self {
            duration: 250,
            current_pos: 1.0,
            start_progress: 0.0,
            end_progress: 1.0,
            animation: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct ChildTransition {
    pub progress: f64,

    pub is_gesture_active: bool,
    pub is_cancelled: bool,

    pub transition_running: bool,
    pub animation: Option<adw::Animation>,

    pub last_visible_widget_width: i32,
    pub last_visible_widget_height: i32,

    pub can_navigate_back: bool,
    pub can_navigate_forward: bool,

    pub active_direction: Option<gtk::PanDirection>,
    pub swipe_direction: i32,
}

#[derive(Debug)]
pub struct ResizeLeaflet {
    pub children: RefCell<Vec<ResizeLeafletPage>>,

    pub visible_child: RefCell<Option<ResizeLeafletPage>>,
    pub last_visible_child: RefCell<Option<ResizeLeafletPage>>,

    pub folded: RefCell<bool>,
    pub fold_threshold_policy: RefCell<adw::FoldThresholdPolicy>,

    pub homogeneous: RefCell<bool>,

    pub orientation: RefCell<gtk::Orientation>,

    pub transition_type: RefCell<adw::LeafletTransitionType>,

    pub tracker: RefCell<Option<adw::SwipeTracker>>,

    pub mode_transition: RefCell<ModeTransition>,

    /* Child transition variables. */
    pub child_transition: RefCell<ChildTransition>,

    pub shadow_helper: RefCell<ShadowHelper>,
    pub can_unfold: RefCell<bool>,

    pub pages: RefCell<Option<gtk::SelectionModel>>,
}

impl Default for ResizeLeaflet {
    fn default() -> Self {
        Self {
            children: RefCell::default(),
            visible_child: RefCell::default(),
            last_visible_child: RefCell::default(),
            folded: RefCell::new(false),
            fold_threshold_policy: RefCell::new(adw::FoldThresholdPolicy::Minimum),
            homogeneous: RefCell::new(true),
            orientation: RefCell::new(gtk::Orientation::Vertical),
            transition_type: RefCell::new(adw::LeafletTransitionType::Over),
            tracker: RefCell::default(),
            mode_transition: RefCell::default(),
            child_transition: RefCell::default(),
            shadow_helper: RefCell::default(),
            can_unfold: RefCell::new(true),
            pages: RefCell::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for ResizeLeaflet {
    const NAME: &'static str = "InoxResizeLeaflet";
    type Type = super::ResizeLeaflet;
    type ParentType = gtk::Widget;
    type Interfaces = (gtk::Orientable, gtk::Buildable, adw::Swipeable);

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for ResizeLeaflet {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecOverride::for_interface::<gtk::Orientable>("orientation"),
                /**
                 * AdwLeaflet:folded: (attributes org.gtk.Property.get=adw_leaflet_get_folded)
                 *
                 * Whether the leaflet is folded.
                 *
                 * The leaflet will be folded if the size allocated to it is smaller than the
                 * sum of the fold threshold policy, it will be unfolded otherwise.
                 *
                 */
                ParamSpecBoolean::new(
                    "folded",
                    "Folded",
                    "Whether the leaflet is folded",
                    false,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:fold-threshold-policy: (attributes org.gtk.Property.get=adw_leaflet_get_fold_threshold_policy org.gtk.Property.set=adw_leaflet_set_fold_threshold_policy)
                 *
                 * Determines when the leaflet will fold.
                 *
                 * If set to `ADW_FOLD_THRESHOLD_POLICY_MINIMUM`, it will only fold when
                 * the children cannot fit anymore. With `ADW_FOLD_THRESHOLD_POLICY_NATURAL`,
                 * it will fold as soon as children don't get their natural size.
                 *
                 * This can be useful if you have a long ellipsizing label and want to let it
                 * ellipsize instead of immediately folding.
                 *
                 */
                ParamSpecEnum::new(
                    "fold-threshold-policy",
                    "Fold Threshold Policy",
                    "Determines when the leaflet will fold",
                    adw::FoldThresholdPolicy::static_type(),
                    0,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:homogeneous: (attributes org.gtk.Property.get=adw_leaflet_get_homogeneous org.gtk.Property.set=adw_leaflet_set_homogeneous)
                 *
                 * Whether the leaflet allocates the same size for all children when folded.
                 *
                 * If set to `FALSE`, different children can have different size along the
                 * opposite orientation.
                 *
                 */
                ParamSpecBoolean::new(
                    "homogeneous",
                    "Homogeneous",
                    "Whether the leaflet allocates the same size for all children when folded",
                    true,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:visible-child: (attributes org.gtk.Property.get=adw_leaflet_get_visible_child org.gtk.Property.set=adw_leaflet_set_visible_child)
                 *
                 * The widget currently visible when the leaflet is folded.
                 *
                 * The transition is determined by [property@Adw.Leaflet:transition-type] and
                 * [Adw.Leaflet:child-transition-duration]. The transition can be cancelled by
                 * the user, in which case visible child will change back to the previously
                 * visible child.
                 *
                 */
                ParamSpecObject::new(
                    "visible-child",
                    "Visible child",
                    "The widget currently visible when the leaflet is folded",
                    gtk::Widget::static_type(),
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:visible-child-name: (attributes org.gtk.Property.get=adw_leaflet_get_visible_child_name org.gtk.Property.set=adw_leaflet_set_visible_child_name)
                 *
                 * The name of the widget currently visible when the leaflet is folded.
                 *
                 * See [property@Adw.Leaflet:visible-child].
                 *
                 */
                ParamSpecString::new(
                    "visible-child-name",
                    "Name of visible child",
                    "The name of the widget currently visible when the leaflet is folded",
                    None,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:transition-type: (attributes org.gtk.Property.get=adw_leaflet_get_transition_type org.gtk.Property.set=adw_leaflet_set_transition_type)
                 *
                 * The type of animation used for transitions between modes and children.
                 *
                 * The transition type can be changed without problems at runtime, so it is
                 * possible to change the animation based on the mode or child that is about
                 * to become current.
                 *
                 */
                ParamSpecEnum::new(
                    "transition-type",
                    "Transition type",
                    "The type of animation used for transitions between modes and children",
                    adw::LeafletTransitionType::static_type(),
                    0,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:mode-transition-duration: (attributes org.gtk.Property.get=adw_leaflet_get_mode_transition_duration org.gtk.Property.set=adw_leaflet_set_mode_transition_duration)
                 *
                 * The mode transition animation duration, in milliseconds.
                 *
                 */
                ParamSpecUInt::new(
                    "mode-transition-duration",
                    "Mode transition duration",
                    "The mode transition animation duration, in milliseconds",
                    0,
                    u32::MAX,
                    250,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:child-transition-params: (attributes org.gtk.Property.get=adw_leaflet_get_child_transition_params org.gtk.Property.set=adw_leaflet_set_child_transition_params)
                 *
                 * The child transition spring parameters.
                 *
                 * The default value is equivalent to:
                 *
                 * ```c
                 * adw_spring_params_new (1, 0.5, 500)
                 * ```
                 *
                 */
                ParamSpecBoxed::new(
                    "child-transition-params",
                    "Child transition parameters",
                    "The child transition spring parameters",
                    adw::SpringParams::static_type(),
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:child-transition-running: (attributes org.gtk.Property.get=adw_leaflet_get_child_transition_running)
                 *
                 * Whether a child transition is currently running.
                 *
                 */
                ParamSpecBoolean::new(
                    "child-transition-running",
                    "Child transition running",
                    "Whether a child transition is currently running",
                    false,
                    ParamFlags::READWRITE,
                ),
                /**
                 * AdwLeaflet:can-navigate-back: (attributes org.gtk.Property.get=adw_leaflet_get_can_navigate_back org.gtk.Property.set=adw_leaflet_set_can_navigate_back)
                 *
                 * Whether gestures and shortcuts for navigating backward are enabled.
                 *
                 * The supported gestures are:
                 * - One-finger swipe on touchscreens
                 * - Horizontal scrolling on touchpads (usually two-finger swipe)
                 * - Back/forward mouse buttons
                 *
                 * The keyboard back/forward keys are also supported, as well as the Alt+←
                 * shortcut for horizontal orientation, or Alt+↑ for vertical
                 * orientation.
                 *
                 * If the orientation is horizontal, for right-to-left locales, gestures and
                 * shortcuts are reversed.
                 *
                 * Only children that have [property@Adw.LeafletPage:navigatable] set to
                 * `TRUE` can be navigated to.
                 *
                 */
                ParamSpecBoolean::new(
                    "can-navigate-back",
                    "Can navigate back",
                    "Whether gestures and shortcuts for navigating backward are enabled",
                    false,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:can-navigate-forward: (attributes org.gtk.Property.get=adw_leaflet_get_can_navigate_forward org.gtk.Property.set=adw_leaflet_set_can_navigate_forward)
                 *
                 * Whether gestures and shortcuts for navigating forward are enabled.
                 *
                 * The supported gestures are:
                 * - One-finger swipe on touchscreens
                 * - Horizontal scrolling on touchpads (usually two-finger swipe)
                 * - Back/forward mouse buttons
                 *
                 * The keyboard back/forward keys are also supported, as well as the Alt+→
                 * shortcut for horizontal orientation, or Alt+↓ for vertical
                 * orientation.
                 *
                 * If the orientation is horizontal, for right-to-left locales, gestures and
                 * shortcuts are reversed.
                 *
                 * Only children that have [property@Adw.LeafletPage:navigatable] set to
                 * `TRUE` can be navigated to.
                 *
                 */
                ParamSpecBoolean::new(
                    "can-navigate-forward",
                    "Can navigate forward",
                    "Whether gestures and shortcuts for navigating forward are enabled",
                    false,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:can-unfold: (attributes org.gtk.Property.get=adw_leaflet_get_can_unfold org.gtk.Property.set=adw_leaflet_set_can_unfold)
                 *
                 * Whether or not the leaflet can unfold.
                 *
                 */
                ParamSpecBoolean::new(
                    "can-unfold",
                    "Can unfold",
                    "Whether or not the leaflet can unfold",
                    true,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                /**
                 * AdwLeaflet:pages: (attributes org.gtk.Property.get=adw_leaflet_get_pages)
                 *
                 * A selection model with the leaflet's pages.
                 *
                 * This can be used to keep an up-to-date view. The model also implements
                 * [iface@Gtk.SelectionModel] and can be used to track and change the visible
                 * page.
                 *
                 */
                ParamSpecObject::new(
                    "pages",
                    "Pages",
                    "A selection model with the leaflet's pages",
                    gtk::SelectionModel::static_type(),
                    ParamFlags::READABLE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            // "expanded" => {
            //     self.expanded.replace(value.get().unwrap());
            // }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            // "expanded" => self.expanded.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        let controller = gtk::GestureClick::new();
        controller.set_button(0);

        controller.connect_pressed(clone!(@weak obj => move |c, n_press, x, y| {
            let this = Self::from_instance(&obj);
            this.on_back_forward_button_pressed(n_press, x, y);
        }));

        obj.add_controller(&controller);

        let tracker = adw::SwipeTracker::new(obj);
        tracker.set_property("orientation", self.orientation.borrow().to_value());
        tracker.set_property("enabled", false.to_value());

        tracker.connect_prepare(clone!(@weak obj => move |t, direction| {
            let this = Self::from_instance(&obj);
            this.on_tracker_prepare(t, direction);
        }));

        tracker.connect_update_swipe(clone!(@weak obj => move |t, progress| {
            let this = Self::from_instance(&obj);
            this.on_tracker_update_swipe(t, progress);
        }));

        tracker.connect_end_swipe(clone!(@weak obj => move |t, velocity, to| {
            let this = Self::from_instance(&obj);
            this.on_tracker_end_swipe(t, velocity, to);
        }));

        self.tracker.replace(Some(tracker));

        //   self->shadow_helper = adw_shadow_helper_new (widget);

        //   gtk_widget_add_css_class (widget, "unfolded");

        //   target = adw_callback_animation_target_new ((AdwAnimationTargetFunc) mode_transition_cb,
        //                                               self, NULL);
        //   self->mode_transition.animation =
        //     adw_timed_animation_new (GTK_WIDGET (self), 0, 1,
        //                              self->mode_transition.duration, target);

        //   target = adw_callback_animation_target_new ((AdwAnimationTargetFunc) child_transition_cb,
        //                                               self, NULL);
        //   self->child_transition.animation =
        //     adw_spring_animation_new (GTK_WIDGET (self), 0, 1,
        //                               adw_spring_params_new (1, 0.5, 500), target);
        //   adw_spring_animation_set_clamp (ADW_SPRING_ANIMATION (self->child_transition.animation),
        //                                   TRUE);
        //   g_signal_connect_swapped (self->child_transition.animation, "done",
        //                             G_CALLBACK (child_transition_done_cb), self);

        self.parent_constructed(obj);
    }

    fn dispose(&self, _obj: &Self::Type) {}
}

impl WidgetImpl for ResizeLeaflet {
    fn measure(
        &self,
        widget: &Self::Type,
        orientation: gtk::Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        let mut visible_children = 0;
        let mut max_min = 0;
        let mut max_nat = 0;
        let mut sum_nat = 0;

        let children = self.children.borrow_mut();

        for page in children.iter() {
            if page.child().is_none() || !page.child().unwrap().is_visible() {
                continue;
            }

            visible_children += 1;

            let (child_min, child_nat, _, _) = page.child().unwrap().measure(orientation, for_size);

            max_min = cmp::max(max_min, child_min);
            max_nat = cmp::max(max_nat, child_nat);
            sum_nat += child_nat;
        }

        let visible_min = if let Some(visible_child) = self.visible_child.borrow().as_ref() {
            if let Some(visible_child_widget) = visible_child.child() {
                let (visible_min, _, _, _) = visible_child_widget.measure(orientation, for_size);
                visible_min
            } else {
                0
            }
        } else {
            0
        };

        let last_visible_min =
            if let Some(last_visible_child) = self.last_visible_child.borrow().as_ref() {
                if let Some(last_visible_child_widget) = last_visible_child.child() {
                    let (last_visible_min, _, _, _) =
                        last_visible_child_widget.measure(orientation, for_size);
                    last_visible_min
                } else {
                    0
                }
            } else {
                visible_min
            };

        let same_orientation = orientation == widget.orientation();

        let minimum = if same_orientation || *self.homogeneous.borrow() {
            max_min
        } else {
            let minimum = lerp(
                last_visible_min as f64,
                visible_min as f64,
                self.child_transition.borrow().progress,
            );
            lerp(
                minimum,
                max_min as f64,
                self.mode_transition.borrow().current_pos,
            ) as i32
        };

        let natural = if same_orientation && *self.can_unfold.borrow() {
            sum_nat
        } else {
            max_nat
        };

        (minimum, natural, -1, -1)
    }

    fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
        let orientation = widget.orientation();

        //   GList *directed_children, *children;
        let mut folded = false;
        let directed_children = self.directed_children();

        // Prepare children information.
        for page in directed_children.iter() {
            if let Some(widget) = page.child() {
                let mut data = page.mut_data();
                let (min, nat) = widget.preferred_size();
                data.min = min;
                data.nat = nat;
                data.alloc = gtk::Allocation::new(0, 0, 0, 0);
                data.visible = false;
            }
        }

        // Check whether the children should be stacked or not.
        if *self.can_unfold.borrow() {
            let mut nat_box_size = 0;
            let mut nat_max_size = 0;
            let mut min_box_size = 0;
            let mut min_max_size = 0;
            let mut visible_children = 0;

            if orientation == gtk::Orientation::Horizontal {
                for page in directed_children.iter() {
                    // FIXME Check the child is visible.
                    if page.child().is_none() {
                        continue;
                    }

                    let data = page.data();

                    if data.nat.width() <= 0 {
                        continue;
                    }

                    nat_box_size += data.nat.width();
                    min_box_size += data.min.width();
                    nat_max_size = cmp::max(nat_max_size, data.nat.width());
                    min_max_size = cmp::max(min_max_size, data.min.width());
                    visible_children += 1;
                }

                if *self.fold_threshold_policy.borrow() == adw::FoldThresholdPolicy::Natural {
                    folded = visible_children > 1 && width < nat_box_size;
                } else {
                    folded = visible_children > 1 && width < min_box_size;
                }
            } else {
                for page in directed_children.iter() {
                    if page.child().is_none() {
                        continue;
                    }

                    let data = page.data();

                    if data.nat.height() <= 0 {
                        continue;
                    }

                    nat_box_size += data.nat.height();
                    min_box_size += data.min.height();
                    nat_max_size = cmp::max(nat_max_size, data.nat.height());
                    min_max_size = cmp::max(min_max_size, data.min.height());
                }

                if *self.fold_threshold_policy.borrow() == adw::FoldThresholdPolicy::Natural {
                    folded = visible_children > 1 && height < nat_box_size;
                } else {
                    folded = visible_children > 1 && height < min_box_size;
                }
            }
        } else {
            folded = true;
        }

        self.set_folded(folded);

        // Allocate size to the children.
        if folded {
            self.size_allocate_folded(width, height);
        } else {
            self.size_allocate_unfolded(width, height);
        }

        // Apply visibility and allocation.
        for page in directed_children.iter() {
            if let Some(widget) = page.child() {
                widget.set_child_visible(page.data().visible)
            }

            if !page.data().visible {
                continue;
            }

            if let Some(widget) = page.child() {
                widget.size_allocate(&page.data().alloc, baseline);

                if widget.is_realized() {
                    widget.show();
                }
            }
        }

        self.allocate_shadow(width, height, baseline);
    }

    fn snapshot(&self, widget: &Self::Type, snapshot: &gtk::Snapshot) {
        let overlap_child = self.top_overlap_child();

        let is_transition = self.child_transition.borrow().transition_running
            || self
                .mode_transition
                .borrow()
                .animation
                .as_ref()
                .map(|animation| animation.state().eq(&adw::AnimationState::Playing))
                .unwrap_or(false);

        if !is_transition
            || *self.transition_type.borrow() == adw::LeafletTransitionType::Slide
            || overlap_child.is_none()
        {
            return self.parent_snapshot(widget, snapshot);
        }

        let stacked_children =
            if *self.transition_type.borrow() == adw::LeafletTransitionType::Under {
                self.children_ref(true)
            } else {
                self.children_ref(false)
            };

        let is_vertical = widget.orientation() == gtk::Orientation::Vertical;
        let is_rtl = widget.direction() == gtk::TextDirection::Rtl;
        let is_over = *self.transition_type.borrow() == adw::LeafletTransitionType::Over;

        let mut shadow_rect = gdk::Rectangle::new(0, 0, widget.width(), widget.height());
        let overlap_child = overlap_child.unwrap();
        if is_vertical {
            if !is_over {
                let y = overlap_child.data().alloc.y() + overlap_child.data().alloc.height();
                shadow_rect = gdk::Rectangle::new(
                    shadow_rect.x(),
                    y,
                    shadow_rect.width(),
                    shadow_rect.height() - y,
                );
            } else {
                shadow_rect = gdk::Rectangle::new(
                    shadow_rect.x(),
                    shadow_rect.y(),
                    shadow_rect.width(),
                    shadow_rect.height() - overlap_child.data().alloc.y(),
                );
            }
        } else if is_over == is_rtl {
            let x = overlap_child.data().alloc.x() + overlap_child.data().alloc.width();
            shadow_rect = gdk::Rectangle::new(
                x,
                shadow_rect.y(),
                shadow_rect.width() - x,
                shadow_rect.height(),
            );
        } else {
            shadow_rect = gdk::Rectangle::new(
                shadow_rect.x(),
                shadow_rect.y(),
                shadow_rect.width() - overlap_child.data().alloc.x(),
                shadow_rect.height(),
            );
        }

        snapshot.push_clip(&graphene::Rect::new(
            shadow_rect.x() as f32,
            shadow_rect.y() as f32,
            shadow_rect.width() as f32,
            shadow_rect.height() as f32,
        ));

        for page in stacked_children {
            if page == overlap_child {
                snapshot.pop();
            }

            if let Some(child) = (page).child().as_ref() {
                widget.snapshot_child(child, snapshot);
            }
        }

        (*self.shadow_helper.borrow()).snapshot(snapshot);
    }

    fn direction_changed(&self, widget: &Self::Type, previous_direction: gtk::TextDirection) {}

    // fn request_mode(&self, widget: &Self::Type) -> gtk::SizeRequestMode {}

    fn compute_expand(&self, widget: &Self::Type, hexpand: &mut bool, vexpand: &mut bool) {}
}

impl OrientableImpl for ResizeLeaflet {}
impl BuildableImpl for ResizeLeaflet {}
impl SwipeableImpl for ResizeLeaflet {}

impl ResizeLeaflet {
    pub fn directed_children(&self) -> Vec<ResizeLeafletPage> {
        let inst = self.instance();
        self.children_ref(
            inst.orientation() == gtk::Orientation::Horizontal
                && inst.direction() == gtk::TextDirection::Rtl,
        )
    }

    pub fn children_ref(&self, reversed: bool) -> Vec<ResizeLeafletPage> {
        if reversed {
            self.children
                .borrow()
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<ResizeLeafletPage>>()
        } else {
            self.children
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<ResizeLeafletPage>>()
        }
    }

    pub fn set_visible_child(&self, child: Option<&ResizeLeafletPage>) {}

    pub fn set_folded(&self, folded: bool) {}

    pub fn size_allocate_folded(&self, width: i32, height: i32) {}

    pub fn size_allocate_unfolded(&self, width: i32, height: i32) {}

    pub fn allocate_shadow(&self, width: i32, height: i32, baseline: i32) {}

    pub fn top_overlap_child(&self) -> Option<ResizeLeafletPage> {
        None
    }

    pub fn on_back_forward_button_pressed(&self, n_press: i32, x: f64, y: f64) {}

    pub fn on_tracker_prepare(
        &self,
        tracker: &adw::SwipeTracker,
        direction: adw::NavigationDirection,
    ) {
    }
    pub fn on_tracker_update_swipe(&self, tracker: &adw::SwipeTracker, progress: f64) {}
    pub fn on_tracker_end_swipe(&self, tracker: &adw::SwipeTracker, velocity: f64, to: f64) {}
}
