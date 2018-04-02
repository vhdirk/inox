use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;

use relm;
use relm::UpdateNew;
use relm_attributes::widget;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;
use constants;
use header::Header;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct MainModel {
    relm: ::relm::Relm<ApplicationWindow>,
    gapp: gtk::Application,
    settings: Rc<Settings>,
    dbmanager: Rc<DBManager>
}


pub struct ApplicationWindow {
    // _button: RelmComponent<Button>,
    // _vbox: RelmContainerComponent<VBox>,
    window: gtk::ApplicationWindow,
    model: MainModel,

}


impl ::relm::Update for ApplicationWindow {

    type Model = MainModel;
    type ModelParam = (gtk::Application, Rc<Settings>, Rc<DBManager>);
    type Msg = Msg;

    // fn model(_: &Relm<Self>, _: ()) -> () {
    //
    //
    // }

    fn model(relm: &::relm::Relm<Self>, (gapp, settings, dbmanager): (gtk::Application, Rc<Settings>, Rc<DBManager>)) -> Self::Model {
        Self::Model {
            relm: relm.clone(),
            gapp,
            settings,
            dbmanager
        }
    }


    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl ::relm::Widget for ApplicationWindow {
    type Root = gtk::ApplicationWindow;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &::relm::Relm<Self>, model: Self::Model) -> Self
    {
        let window = gtk::ApplicationWindow::new(&model.gapp);

        let header = ::relm::create_component::<Header, Self>(relm, ());

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), gtk::Inhibit(false)));

        window.set_default_size(800, 600);

        // // Set the headerbar as the title bar widget.
        window.set_titlebar(header.widget());
        // Set the title of the window.
        window.set_title(constants::APPLICATION_NAME);
        // Set the window manager class.
        window.set_wmclass(constants::APPLICATION_CLASS, constants::APPLICATION_NAME);
        // The icon the app will display.
        gtk::Window::set_default_icon_name(constants::APPLICATION_ICON_NAME);

        window.show_all();
        ApplicationWindow {
            window: window,
            model: model
        }
    }
}
