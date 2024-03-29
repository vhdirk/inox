
use crate::core::util::EmptyOrWhitespace;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::{ParamFlags, ParamSpec, ParamSpecString, Value};
use gtk::{prelude::*, subclass::prelude::*, CompositeTemplate};

pub const CLASS_HAS_TEXT: &str = "inox-has-text";

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/com/github/vhdirk/Inox/gtk/placeholder_pane.ui")]
pub struct PlaceholderPane {
    #[template_child]
    pub placeholder_image: TemplateChild<gtk::Image>,

    #[template_child]
    pub title_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub subtitle_label: TemplateChild<gtk::Label>,
}

impl PlaceholderPane {
    pub fn update(&self) {
        let inst = self.instance();

        if self.title_label.get().text().is_empty_or_whitespace() {
            self.title_label.hide();
        }
        if self.subtitle_label.get().text().is_empty_or_whitespace() {
            self.subtitle_label.hide();
        }
        if (self.title_label.get().is_visible() || self.subtitle_label.get().is_visible()) {
            inst.style_context().add_class(CLASS_HAS_TEXT);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for PlaceholderPane {
    const NAME: &'static str = "InoxPlaceholderPane";
    type Type = super::PlaceholderPane;
    type ParentType = gtk::Grid;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PlaceholderPane {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;

        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::new(
                    // Name
                    "icon-name",
                    // Nickname
                    "icon name",
                    // Short description
                    "Icon name",
                    // Default value
                    None,
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
                ParamSpecString::new(
                    // Name
                    "title",
                    // Nickname
                    "Title",
                    // Short description
                    "Title",
                    // Default value
                    None,
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
                ParamSpecString::new(
                    // Name
                    "subtitle",
                    // Nickname
                    "subtitle",
                    // Short description
                    "subtitle",
                    // Default value
                    None,
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "icon-name" => {
                let icon_name = value.get().unwrap();
                self.placeholder_image.get().set_icon_name(icon_name);
            }
            "title" => {
                let title = value.get().unwrap();
                self.title_label.get().set_text(title);
            }
            "subtitle" => {
                let subtitle = value.get().unwrap();
                self.subtitle_label.get().set_text(subtitle);
            }
            _ => unimplemented!(),
        }
        self.update();
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "icon-name" => self.placeholder_image.get().icon_name().to_value(),
            "title" => self.title_label.get().text().to_value(),
            "subtitle" => self.subtitle_label.get().text().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for PlaceholderPane {}
impl GridImpl for PlaceholderPane {}
