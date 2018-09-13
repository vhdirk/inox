use gio::MenuModel;
use gtk;
use gtk::prelude::*;

use crossbeam_channel::Sender;
use failure::Error;
use rayon;
// use url::Url;
use relm::{Relm, Update, Widget, WidgetTest};

use uibuilder::UI;

use app::Action;
use stacks::Content;
// use utils::{itunes_to_rss, refresh};

use std::rc::Rc;



#[derive(Msg)]
pub enum Msg {
    Change,
    Quit,
}


pub struct Model{
    ui: UI,
}

struct Widgets{}

pub struct HeaderBar {
    model: Model,
    container: gtk::Box,
    widgets: Widgets
    // switch: gtk::StackSwitcher,
    // back: gtk::Button,
    // show_title: gtk::Label,
    // menu_button: gtk::MenuButton,
    // app_menu: MenuModel,
    // updater: UpdateIndicator,    
    // add: AddPopover,
}


// TODO: Factor out the hamburger menu
// TODO: Make a proper state machine for the headerbar states
impl HeaderBar {
    // pub fn new(ui: UI) -> Rc<Self> {
    //     let h = Rc::new(Self{
    //         ui: ui.clone(),
    //         container: ui.builder.get_object("main_header")
    //             .expect("Couldn't find main_header in ui file."),
    //     });
    //     Self::init(&h/*, content, &sender*/);
    //     h
    // }

    // pub fn init(s: &Rc<Self>/*, content: &Content, sender: &Sender<Action>*/) {
    //     let weak = Rc::downgrade(s);

        //s.switch.set_stack(&content.get_stack());

        // s.add.entry.connect_changed(clone!(weak => move |_| {
        //     weak.upgrade().map(|h| {
        //         h.add.on_entry_changed()
        //             .map_err(|err| error!("Error: {}", err))
        //             .ok();
        //     });
        // }));
        //
        // s.add.add.connect_clicked(clone!(weak, sender => move |_| {
        //     weak.upgrade().map(|h| h.add.on_add_clicked(&sender));
        // }));

        // let switch = &s.switch;
        // let add_toggle = &s.add.toggle;
        // let show_title = &s.show_title;
        // let menu = &s.menu_button;
        // s.back.connect_clicked(
        //     clone!(switch, add_toggle, show_title, sender, menu => move |back| {
        //         switch.show();
        //         add_toggle.show();
        //         back.hide();
        //         show_title.hide();
        //         menu.show();
        //         sender.send(Action::ShowShowsAnimated);
        //     }),
        // );

       // s.menu_button.set_menu_model(Some(&s.app_menu));
    //}

    // pub fn switch_to_back(&self, title: &str) {
    //     self.switch.hide();
    //     // self.add.toggle.hide();
    //     self.back.show();
    //     self.set_show_title(title);
    //     self.show_title.show();
    //     self.menu_button.hide();
    // }

    // pub fn switch_to_normal(&self) {
    //     self.switch.show();
    //     // self.add.toggle.show();
    //     self.back.hide();
    //     self.show_title.hide();
    //     self.menu_button.show();
    // }

    // pub fn set_show_title(&self, title: &str) {
    //     self.show_title.set_text(title)
    // }

    // pub fn show_update_notification(&self) {
    //     // self.updater.show();
    // }

    // pub fn hide_update_notification(&self) {
    //     // self.updater.hide();
    // }

    // pub fn open_menu(&self) {
    //     self.menu_button.clicked();
    // }
}

impl Update for HeaderBar{
    type Model = Model;
    type ModelParam = (UI,);
    type Msg = Msg;


    fn model(r: &Relm<Self>, (ui,): Self::ModelParam) -> Model {
        Self::Model {
            ui
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Change => {
                // self.model.content = self.widgets.input.get_text()
                //                                        .expect("get_text failed")
                //                                        .chars()
                //                                        .rev()
                //                                        .collect();
                // self.widgets.label.set_text(&self.model.content);
            },
            Quit => gtk::main_quit(),
        }
    }
}

impl Widget for HeaderBar {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.container.clone()
    }

    fn view(r: &Relm<Self>, model: Self::Model) -> Self {
        
        let container = model.ui.builder.get_object::<gtk::Box>("main_header")
                               .expect("Couldn't find main_header in ui file.");
        HeaderBar {
            model,
            container,
            widgets: Widgets{}
        }

    }
}