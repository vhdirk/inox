use std::ops::AddAssign;
use std::str::FromStr;
use glib;
use gdk;
use md5;

pub trait ToHex{
    fn to_hex(&self) -> String;
}


impl ToHex for gdk::RGBA
{
    fn to_hex(&self) -> String{
        let hex = format!("#{:02x}{:02x}{:02x}{:02x}",
            (self.red * 255.0 / 65535.0) as u8,
            (self.green * 255.0 / 65535.0) as u8,
            (self.blue * 255.0 / 65535.0) as u8,
            (self.alpha * 255.0) as u8
        );
        hex.to_string()
    }
}

pub fn get_tag_color_rgba(tag: &String, canvascolor: &gdk::RGBA) -> (gdk::RGBA, gdk::RGBA)
{
    //TODO: get from settings
    let tags_upper_color = gdk::RGBA::from_str(&"#e5e5e5".to_string()).unwrap();
    let tags_lower_color = gdk::RGBA::from_str(&"#333333".to_string()).unwrap();
    let tags_alpha = 0.5;

    // # ifndef DISABLE_PLUGINS
    //
    // Gdk::RGBA canvas;
    // canvas.set_red_u   ( cv[0] * 65535 / 255 );
    // canvas.set_green_u ( cv[1] * 65535 / 255 );
    // canvas.set_blue_u  ( cv[2] * 65535 / 255 );
    //
    // auto clrs = astroid->plugin_manager->astroid_extension->get_tag_colors (t, rgba_to_hex (canvas));
    //
    // if (!clrs.first.empty () || !clrs.second.empty ()) {
    //   return std::make_pair (Gdk::RGBA (clrs.first), Gdk::RGBA (clrs.second));
    // }
    // # endif

    let tc = md5::compute(tag);

    /*
     * normalize the background tag color to be between upper and
     * lower, then choose light or dark font color depending on
     * luminocity of background color.
     */

    let bg = gdk::RGBA{
        red: tc[0] as f64 * (tags_upper_color.red - tags_lower_color.red) + tags_lower_color.red,
        green: tc[1] as f64 * (tags_upper_color.green - tags_lower_color.green) + tags_lower_color.green,
        blue: tc[2] as f64 * (tags_upper_color.blue - tags_lower_color.blue) + tags_lower_color.blue,
        alpha: 0.0
    };

    let bc = gdk::RGBA{
        red: bg.red * (65535.0) / (255.0),
        green: bg.green * (65535.0) / (255.0),
        blue: bg.blue * (65535.0) / (255.0),
        alpha: (tags_alpha * (65535.0))
    };

    let mut lum: f64 = ((bg.red * tags_alpha + (1.0 - tags_alpha) * canvascolor.red ) * 0.2126 +
                       (bg.green * tags_alpha + (1.0 - tags_alpha) * canvascolor.green) * 0.7152 +
                       (bg.blue * tags_alpha + (1.0 - tags_alpha) * canvascolor.blue) * 0.0722) / 255.0;
    /* float avg = (bg[0] + bg[1] + bg[2]) / (3 * 255.0); */


    let mut fc = gdk::RGBA::from_str("#f2f2f2").unwrap();
    if lum > 0.5 {
      fc = gdk::RGBA::from_str("#000000").unwrap();
    }

    (fc, bc)
  }



pub fn get_tag_color (tag: &String, canvascolor: &gdk::RGBA) -> (String, String){
  let clrs = get_tag_color_rgba(&tag, &canvascolor);

  (clrs.0.to_hex(), clrs.1.to_hex())
}


pub fn concat_tags_color(tags: &Vec<String>,
                         use_pango: bool,
                         maxlen: i32,
                         canvascolor: &gdk::RGBA) ->String
{

    let mut tag_string = "".to_string();
    let mut first = true;
    let mut broken = false;
    let mut len = 0;

    let tags_alpha = 0.5;

    for t in tags {
        let mut tag = t.clone();

        if !first {
            if use_pango {
                tag_string.add_assign("<span size=\"xx-small\"> </span>");
            } else {
                tag_string.add_assign(" ");
            }
        } else {
            first = false;
        }

        let mut colors = get_tag_color(&tag, &canvascolor);

        if maxlen > 0 {
            broken = true;
            if len >= maxlen{ break; }
            broken = false;

            if (len + tag.len() as i32 + 2) > maxlen {
                let cur_tag_len = tag.len() as i32;
                tag.truncate((len + cur_tag_len + 2 - maxlen) as usize);
                tag.add_assign("..");
            }

            len += tag.len() as i32 + 2;
        }

        if use_pango {
            tag_string.add_assign(&format!("<span bgcolor=\"{}\" color=\"{}\"> {} </span>",
                                  colors.1,
                                  colors.0,
                                  glib::markup_escape_text(tag.as_str())));

        } else {
            colors.1.truncate(7);
            let mut bg =  gdk::RGBA::from_str(&colors.1).unwrap();
            bg.alpha = tags_alpha;

            tag_string.add_assign(&format!("<span style=\"background-color: rgba({}, {}, {}, {}); color: {} !important; white-space: pre;\"> {} </span>",
                                (bg.red * 255.0) as u8,
                                (bg.green * 255.0) as u8,
                                (bg.blue * 255.0) as u8,
                                bg.alpha,
                                colors.0,
                                glib::markup_escape_text(tag.as_str())));
        }

    }
    if broken {
        tag_string.add_assign("..");
    }
    tag_string
}
