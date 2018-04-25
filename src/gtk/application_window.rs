use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;

use relm;
use relm::{UpdateNew, ContainerWidget};
use relm_attributes::widget;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;
use constants;
use header::Header;
use header::Msg as HeaderMsg;

use main_content::{MainContent, Msg as MainContentMsg};
use tag_list::{TagList, Msg as TagListMsg};
use thread_list::{ThreadList, Msg as ThreadListMsg};
use thread_view::ThreadView;



#[derive(Msg)]
pub enum Msg {
    TagSelect(Option<String>),
    ThreadSelect(Option<String>), //emitted when the currently selected thread is changed
    Quit,
}

pub struct MainModel {
    relm: ::relm::Relm<ApplicationWindow>,
    gapp: gtk::Application,
    settings: Rc<Settings>,
    dbmanager: Arc<DBManager>,
    ui_orientation: gtk::Orientation
}


use self::TagListMsg::ItemSelect as TagList_ItemSelect;
use self::ThreadListMsg::ThreadSelect as ThreadList_ThreadSelect;

use self::Msg::TagSelect as TagSelect;
use self::Msg::ThreadSelect as ThreadSelect;


#[widget]
impl ::relm::Widget for ApplicationWindow {
    type Root = gtk::ApplicationWindow;
    type Model = MainModel;
    type ModelParam = (gtk::Application, Rc<Settings>, Arc<DBManager>);
    type Msg = Msg;

    fn on_tag_changed(self: &mut Self, tag:Option<String>){


        // TODO: build a new query and refresh the thread list.

        let qs = match tag{
            Some(tag) => format!("tag:{}", tag).to_string(),
            None => "".to_string()
        };
        debug!("qs: {:?}", qs);


        self.thread_list.emit(ThreadListMsg::Update(qs));
    }

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn init_view(&mut self) {
        // The icon the app will display.
        gtk::Window::set_default_icon_name(constants::APPLICATION_ICON_NAME);

        self.window.set_default_size(800, 600);

        // // Set the headerbar as the title bar widget.
        // self.window.set_titlebar(self.header.widget());
        // Set the title of the window.
        self.window.set_title(constants::APPLICATION_NAME);
        // Set the window manager class.
        self.window.set_wmclass(constants::APPLICATION_CLASS, constants::APPLICATION_NAME);

        self.window.set_role(constants::APPLICATION_CLASS);



        self.thread_container.set_size_request(100, -1);
        self.tag_list.widget().set_size_request(100, -1);
        self.thread_list.widget().set_size_request(100, -1);
        self.thread_view.widget().set_size_request(100, -1);

        self.tag_list.emit(TagListMsg::Refresh);

        self.window.show_all();
        self.on_tag_changed(None);

    }


    fn model(relm: &::relm::Relm<Self>, (gapp, settings, dbmanager): (gtk::Application, Rc<Settings>, Arc<DBManager>)) -> MainModel {
        MainModel {
            relm: relm.clone(),
            gapp,
            settings,
            dbmanager,
            ui_orientation: gtk::Orientation::Horizontal,
        }
    }


    fn update(&mut self, event: Msg) {
        match event {
            Msg::ThreadSelect(ref thread_id) => {
                // self.header.emit(HeaderMsg::ThreadSelect(thread_id.clone()));
                debug!("select thread: {:?}", thread_id);

            },
            Msg::Quit => gtk::main_quit(),
        }
    }

    // fn view(relm: &::relm::Relm<Self>, model: Self::Model) -> Self
    // {
    //     let window = gtk::ApplicationWindow::new(&model.gapp);
    //
    //     let header = ::relm::create_component::<Header>(());
    //
    //     // Connect the signal `delete_event` to send the `Quit` message.
    //     connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), gtk::Inhibit(false)));
    //
    //     let content = window.add_widget::<MainContent>((model.settings.clone(), model.dbmanager.clone()));
    //
    //
    //     use self::MainContentMsg::ThreadSelect as MainContent_ThreadSelect;
    //     use self::HeaderMsg::ThreadSelect as Header_ThreadSelect;
    //
    //     connect!(content@MainContent_ThreadSelect(ref thread_id), header, Header_ThreadSelect(thread_id.clone()));
    //
    //
    //     ApplicationWindow {
    //         window: window,
    //         model: model,
    //         header: header,
    //         content: content
    //     }
    // }



    view! {
        #[name="window"]
        gtk::ApplicationWindow(&self.model.gapp){
            titlebar: Header,

            #[name="container"]
            gtk::Paned(self.model.ui_orientation) {
                #[name="tag_list"]
                TagList((self.model.settings.clone(), self.model.dbmanager.clone())){
                    TagList_ItemSelect(ref tag) => TagSelect(tag.clone())
                },

                #[name="thread_container"]
                gtk::Paned(self.model.ui_orientation){
                    #[name="thread_list"]
                    ThreadList(self.model.settings.clone(), self.model.dbmanager.clone()) {
                        ThreadList_ThreadSelect(ref thread_id) => ThreadSelect(thread_id.clone()),
                    },
                    #[name="thread_view"]
                    ThreadView
                }
            }
        }
    }

}
