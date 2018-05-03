use std::sync::{Once, ONCE_INIT};
use std::cell::{RefCell};
use std::ptr;
use std::cmp::max;
use std::str::FromStr;

use glib;
use gtk;
use gdk;
use gdk::ContextExt;
use gdk_pixbuf;
use gdk_pixbuf::prelude::*;
use cairo;
use pangocairo;
use gobject_ffi;
use glib::translate::*;
use gtk::prelude::*;
use chrono;

use pango;
use pango::prelude::*;
use pango::ContextExt as PangoContextExt;
use pango::LayoutExt;
use glib::value::AnyValue;
use notmuch;


use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;

use notmuch::DatabaseMode;

use gobject_subclass::object::*;
use gobject_subclass::properties::*;

use cell_renderer::*;


pub trait CellRendererThreadImpl: 'static {

}

pub struct CellRendererThreadCache{
    height_set: bool,
    marked: bool,
    height: i32,

    content_height: i32,
    line_height: i32,
    left_icons_size: i32,
    left_icons_width: i32,

    date_start: i32,
    date_width: i32,

    message_count_start: i32,
    message_count_width: i32,

    authors_start: i32,
    authors_width: i32,

    tags_start: i32,
    tags_width: i32,

    subject_start: i32,

    flagged_icon: Option<gdk_pixbuf::Pixbuf>,
    attachment_icon: Option<gdk_pixbuf::Pixbuf>

}


impl Default for CellRendererThreadCache{

    fn default() -> Self{
        CellRendererThreadCache{
            height_set: false,
            marked: false,
            height: 0,

            line_height: 0,
            content_height: 0,
            left_icons_size: 0,
            left_icons_width: 0,

            date_start: 0,
            date_width: 0,

            message_count_start: 0,
            message_count_width: 0,

            authors_start: 0,
            authors_width: 0,

            tags_start: 0,
            tags_width: 0,

            subject_start: 0,
            flagged_icon: None,
            attachment_icon: None
        }
    }
}


pub struct CellRendererThreadSettings {
    // cached values

    date_len: i32,

    message_count_len: i32,

    authors_len: i32,

    tags_len: i32,

    left_icons_width_n: i32,
    left_icons_padding: i32,
    font_description: pango::FontDescription,
    language: pango::Language,
    line_spacing : i32,
    date_length : i32,
    message_count_length : i32,
    authors_length : i32,

    subject_color: Option<String>,
    subject_color_selected : Option<String>,
    background_color_selected : Option<String>,
    background_color_marked : Option<String>,
    background_color_marked_selected : Option<String>,

    tags_length : u16,
    tags_upper_color : Option<String>,
    tags_lower_color : Option<String>,
    tags_alpha : f32,
    hidden_tags : Vec<String>
}

impl Default for CellRendererThreadSettings{

    fn default() -> Self{

        CellRendererThreadSettings
        {
            left_icons_width_n: 2,
            left_icons_padding: 1,
            date_len: 10,
            message_count_len: 4,
            authors_len: 20,
            tags_len: 80,

            language: pango::Language::default(),
            font_description : pango::FontDescription::from_string("default"),
            line_spacing : 2,
            date_length : 10,
            message_count_length : 4,
            authors_length : 20,

            subject_color : Some("#807d74".to_string()),
            subject_color_selected : Some("#000000".to_string()),
            background_color_selected : None,
            background_color_marked : Some("#fff584".to_string()),
            background_color_marked_selected : Some("#bcb559".to_string()),

            tags_length : 80,
            tags_upper_color : Some("#e5e5e5".to_string()),
            tags_lower_color : Some("#333333".to_string()),
            tags_alpha : 0.5,
            hidden_tags : ["attachment".to_string(),
                           "flagged".to_string(),
                           "unread".to_string()].to_vec(),
        }
    }
}










pub struct CellRendererThread {
    thread: RefCell<Option<notmuch::Thread>>,
    settings: RefCell<CellRendererThreadSettings>,
    cache: RefCell<CellRendererThreadCache>,
}


static PROPERTIES: [Property; 1] = [
    Property::Boxed(
        "thread",
        "Thread to display",
        "Handle of notmuch::Thread to display",
        AnyValue::static_type,
        PropertyMutability::ReadWrite,
    ),


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
            let obj: glib::Object = from_glib_none(gobject_ffi::g_object_newv(TYPE.to_glib(), 0, ptr::null_mut()));
            obj.downcast_unchecked()

            //glib::Object::new(TYPE, &[]).unwrap().downcast_unchecked()
        }
    }

    fn class_init(klass: &mut CellRendererClass) {
        klass.install_properties(&PROPERTIES);
    }

    fn init(_renderer: &CellRenderer) -> Box<CellRendererImpl<CellRenderer>>
    {
        let imp = Self{
            thread: RefCell::new(None),
            settings: RefCell::new(CellRendererThreadSettings::default()),
            cache: RefCell::new(CellRendererThreadCache::default()),
        };
        Box::new(imp)
    }

    fn calculate_height(&self, widget: &gtk::Widget)
    {
        let settings = self.settings.borrow();
        let mut cache = self.cache.borrow_mut();

        let pango_cr = widget.create_pango_context().unwrap();
        let font_metrics = pango_cr.get_metrics(&settings.font_description, &settings.language).unwrap();

        let mut char_width = font_metrics.get_approximate_char_width() / pango::SCALE;
        if char_width == 0{
            char_width = 2;
        }
        debug!("char width: {:?}", font_metrics.get_approximate_char_width());
        let padding = char_width;

        /* figure out font height */
        let pango_layout = widget.create_pango_layout("TEST HEIGHT STRING").unwrap();

        pango_layout.set_font_description(&settings.font_description);

        let (_, h) = pango_layout.get_pixel_size();

        cache.content_height = h;

        cache.line_height = cache.content_height + settings.line_spacing;

        cache.height_set = true;

        cache.left_icons_size  = cache.content_height - (2 * settings.left_icons_padding);
        cache.left_icons_width = cache.left_icons_size;

        cache.date_start          = settings.left_icons_width_n * cache.left_icons_width +
             (settings.left_icons_width_n-1) * settings.left_icons_padding + padding;
        cache.date_width          = char_width * settings.date_len;
        cache.message_count_width = char_width * settings.message_count_len;
        cache.message_count_start = cache.date_start + cache.date_width + padding;
        cache.authors_width       = char_width * settings.authors_len;
        cache.authors_start       = cache.message_count_start + cache.message_count_width + padding;
        cache.tags_width          = char_width * settings.tags_len;
        cache.tags_start          = cache.authors_start + cache.authors_width + padding;
        cache.subject_start       = cache.tags_start + cache.tags_width + padding;

        cache.height              = cache.content_height + settings.line_spacing;
    }

    fn render_background(&self, _renderer: &CellRenderer,
                                cr: &cairo::Context,
                                _widget: &gtk::Widget,
                                background_area: &gtk::Rectangle,
                                _cell_area: &gtk::Rectangle,
                                flags: gtk::CellRendererState)
    {
        let settings = &self.settings.borrow();
        let cache = &self.cache.borrow();

        let mut bg: gdk::RGBA = gdk::RGBA::from_str("#ffffff").unwrap();
        let mut set = true;

        if flags.contains(gtk::CellRendererState::SELECTED){
            if !cache.marked {
                match settings.background_color_selected.as_ref() {
                    Some(ref color) => {
                        bg = gdk::RGBA::from_str(color.as_str()).unwrap();
                    },
                    None => {
                        set = false;
                    }
                }
            } else {
                bg = gdk::RGBA::from_str(settings.background_color_marked_selected.as_ref().unwrap().as_str()).unwrap();
            }
        } else {
            if !cache.marked {
                set = false;
            } else {
                bg = gdk::RGBA::from_str(settings.background_color_marked.as_ref().unwrap().as_str()).unwrap();
            }
        }

        if set {
            cr.set_source_rgba(bg.red, bg.green, bg.blue, bg.alpha);

            cr.rectangle(background_area.x.into(), background_area.y.into(), background_area.width.into(), background_area.height.into());
            cr.fill();
        }
   }

   fn render_subject(&self, _renderer: &CellRenderer,
                            cr: &cairo::Context,
                            widget: &gtk::Widget,
                            _background_area: &gtk::Rectangle,
                            cell_area: &gtk::Rectangle,
                            flags: gtk::CellRendererState)
   {
        let settings = &self.settings.borrow();
        let cache = &self.cache.borrow();

        let pango_layout = widget.create_pango_layout("").unwrap();

        pango_layout.set_font_description(&settings.font_description);

           /* set color */
        let stylecontext = widget.get_style_context().unwrap();
        let color = stylecontext.get_color(gtk::StateFlags::NORMAL);

        cr.set_source_rgba(color.red, color.green, color.blue, color.alpha);

        let mut color_str = "".to_string();
        if flags.contains(gtk::CellRendererState::SELECTED) {
            color_str = settings.subject_color_selected.as_ref().unwrap().clone();
        } else {
            color_str = settings.subject_color.as_ref().unwrap().clone();
        }

        pango_layout.set_markup(format!("<span color=\"{}\">{}</span>",
               color_str,
               glib::markup_escape_text(self.thread.borrow().as_ref().unwrap().subject().as_str())).as_str());

        /* align in the middle */
        let (_, h) = pango_layout.get_size();
        let y = max(0,(cache.line_height / 2) - ((h / pango::SCALE) / 2));

        cr.move_to((cell_area.x + cache.subject_start) as f64, (cell_area.y + y) as f64);
        pangocairo::functions::show_layout(&cr, &pango_layout);
    }

    fn render_icon(&self, _renderer: &CellRenderer,
                          settings: &CellRendererThreadSettings,
                          cache:  &CellRendererThreadCache,
                          cr: &cairo::Context,
                          widget: &gtk::Widget,
                          background_area: &gtk::Rectangle,
                          cell_area: &gtk::Rectangle,
                          flags: gtk::CellRendererState,
                          icon_name: &str,
                          icon_cache: &mut Option<gdk_pixbuf::Pixbuf>,
                          icon_offset: i32)
    {
        //
        // if icon_cache.is_none() {
        //     let theme = gtk::IconTheme::get_default().unwrap();
        //     let pixbuf = theme.load_icon(icon_name,
        //                     cache.left_icons_size,
        //                     gtk::IconLookupFlags::USE_BUILTIN | gtk::IconLookupFlags::FORCE_SIZE)
        //                     .unwrap()
        //                     .unwrap();
        //
        //     *icon_cache = pixbuf.scale_simple(cache.left_icons_size,
        //                                       cache.left_icons_size,
        //                                       gdk_pixbuf::InterpType::Bilinear);
        // }
        //
        // let y = cell_area.y + settings.left_icons_padding + settings.line_spacing / 2;
        // let x = cell_area.x + icon_offset * (cache.left_icons_width + settings.left_icons_padding);
        //
        // cr.set_source_pixbuf(icon_cache.as_ref().unwrap(), x as f64, y as f64);
        //
        // cr.rectangle(x as f64, y as f64, cache.left_icons_size as f64, cache.left_icons_size as f64);
        // cr.fill();

    }


    fn render_flagged(&self, renderer: &CellRenderer,
                             cr: &cairo::Context,
                             widget: &gtk::Widget,
                             background_area: &gtk::Rectangle,
                             cell_area: &gtk::Rectangle,
                             flags: gtk::CellRendererState)
    {
        let settings = self.settings.borrow();
        let mut cache = self.cache.borrow_mut();

        let icon_name = "starred-symbolic";
        let icon_offset = 0;

        if cache.flagged_icon.is_none() {
            let theme = gtk::IconTheme::get_default().unwrap();
            let pixbuf = theme.load_icon(icon_name,
                            cache.left_icons_size,
                            gtk::IconLookupFlags::USE_BUILTIN | gtk::IconLookupFlags::FORCE_SIZE)
                            .unwrap()
                            .unwrap();

            cache.flagged_icon = pixbuf.scale_simple(cache.left_icons_size,
                cache.left_icons_size,
                gdk_pixbuf::InterpType::Bilinear);
        }

        let y = cell_area.y + settings.left_icons_padding + settings.line_spacing / 2;
        let x = cell_area.x + icon_offset * (cache.left_icons_width + settings.left_icons_padding);

        cr.set_source_pixbuf(cache.flagged_icon.as_ref().unwrap(), x as f64, y as f64);

        cr.rectangle(x as f64, y as f64, cache.left_icons_size as f64, cache.left_icons_size as f64);
        cr.fill();
    }


    fn render_attachment(&self, renderer: &CellRenderer,
                                cr: &cairo::Context,
                                widget: &gtk::Widget,
                                background_area: &gtk::Rectangle,
                                cell_area: &gtk::Rectangle,
                                flags: gtk::CellRendererState)
    {
        let settings = self.settings.borrow();
        let mut cache = self.cache.borrow_mut();

        let icon_name = "mail-attachment-symbolic";
        let icon_offset = 1;

        if cache.attachment_icon.is_none() {
         let theme = gtk::IconTheme::get_default().unwrap();
         let pixbuf = theme.load_icon(icon_name,
                         cache.left_icons_size,
                         gtk::IconLookupFlags::USE_BUILTIN | gtk::IconLookupFlags::FORCE_SIZE)
                         .unwrap()
                         .unwrap();

         cache.attachment_icon = pixbuf.scale_simple(cache.left_icons_size,
             cache.left_icons_size,
             gdk_pixbuf::InterpType::Bilinear);
        }

        let y = cell_area.y + settings.left_icons_padding + settings.line_spacing / 2;
        let x = cell_area.x + icon_offset * (cache.left_icons_width + settings.left_icons_padding);

        cr.set_source_pixbuf(cache.attachment_icon.as_ref().unwrap(), x as f64, y as f64);

        cr.rectangle(x as f64, y as f64, cache.left_icons_size as f64, cache.left_icons_size as f64);
        cr.fill();

    }

    fn render_delimiter(&self, renderer: &CellRenderer,
                                cr: &cairo::Context,
                                widget: &gtk::Widget,
                                background_area: &gtk::Rectangle,
                                cell_area: &gtk::Rectangle,
                                flags: gtk::CellRendererState)
    {
        let settings = self.settings.borrow();
        let mut cache = self.cache.borrow_mut();

        cr.set_line_width(0.5);
        cr.set_source_rgb(0.1, 0.1, 0.1);
        cr.move_to(cell_area.x as f64, cell_area.y as f64 + cell_area.height as f64);
        cr.line_to((cell_area.x + cell_area.width) as f64, (cell_area.y + cell_area.height) as f64);
        cr.stroke();
    }

    fn render_date(&self, renderer: &CellRenderer,
                          cr: &cairo::Context,
                          widget: &gtk::Widget,
                          background_area: &gtk::Rectangle,
                          cell_area: &gtk::Rectangle,
                          flags: gtk::CellRendererState) -> i32
    {
        use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Local};

        let settings = self.settings.borrow();
        let mut cache = self.cache.borrow_mut();

        let timestamp = self.thread.borrow().as_ref().unwrap().newest_date();
        let datetime_utc = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
        let datetime = datetime_utc.with_timezone(&Local);

        let datestr = format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));

        let pango_layout = widget.create_pango_layout(datestr.as_str()).unwrap();

        pango_layout.set_font_description(&settings.font_description);

        /* set color */
        let stylecontext = widget.get_style_context().unwrap();
        let color = stylecontext.get_color(gtk::StateFlags::NORMAL);
        cr.set_source_rgb(color.red, color.green, color.blue);

        /* align in the middle */
        let (w, h) = pango_layout.get_size();
        let y = max(0, (cache.line_height / 2) - ((h / pango::SCALE) / 2));

        /* update subject start */
        //subject_start = date_start + (w / Pango::SCALE) + padding;

        cr.move_to((cell_area.x + cache.date_start) as f64, (cell_area.y + y) as f64);
        pangocairo::functions::show_layout(&cr, &pango_layout);

        return h;
    }


}



impl ObjectImpl<CellRenderer> for CellRendererThread
{
    fn set_property(&self, _obj: &glib::Object, id: u32, value: &glib::Value) {
        let prop = &PROPERTIES[id as usize];

        match *prop {
            Property::Boxed("thread", ..) => {
                let any_v = value.get::<&AnyValue>().expect("Value did not actually contain an AnyValue");
                *(self.thread.borrow_mut()) = Some(any_v.downcast_ref::<notmuch::Thread>().unwrap().clone());
            },
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, _obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
        let prop = &PROPERTIES[id as usize];

        match *prop {
            Property::Boxed("thread", ..) => {
                Ok("1".to_value())
            },
            _ => unimplemented!(),
        }
    }
}

impl CellRendererImpl<CellRenderer> for CellRendererThread {

    fn render(&self,
        renderer: &CellRenderer,
        cr: &cairo::Context,
        widget: &gtk::Widget,
        background_area: &gtk::Rectangle,
        cell_area: &gtk::Rectangle,
        flags: gtk::CellRendererState,
    ){

        // calculate text width, we don't need to do this every time,
        // but we need access to the context.
        if  !self.cache.borrow().height_set {
            self.calculate_height(widget);
        }

        if self.thread.borrow().is_none(){
            return;
        }

        let thread = self.thread.borrow().as_ref().unwrap().clone();

        /*if thread.unread() {
          settings.font_description.set_weight(pango::Weight::Bold);
        } else */ {
          self.settings.borrow_mut().font_description.set_weight(pango::Weight::Normal);
        }

        self.render_background(&renderer, &cr, &widget, &background_area, &cell_area, flags);
        self.render_date(&renderer, &cr, &widget, &background_area, &cell_area, flags); // returns height

        if thread.total_messages() > 1 {
          //render_message_count (cr, widget, cell_area);
        }

        // render_authors (cr, widget, cell_area);
        //
        // tags_width = render_tags (cr, widget, cell_area, flags); // returns width
        // subject_start = tags_start + tags_width / Pango::SCALE + ((tags_width > 0) ? padding : 0);
        //
        self.render_subject(&renderer, &cr, &widget, &background_area, &cell_area, flags);


        // if (thread->flagged)
        //   render_flagged (cr, widget, cell_area);
        //
        // if (thread->attachment)
        //   render_attachment (cr, widget, cell_area);
        //
        // /*
        // if (marked)
        //   render_marked (cr, widget, cell_area);
        // */

    }

}

#[derive(Default)]
struct CellRendererThreadStatic{}


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
