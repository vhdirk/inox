#![allow(new_without_default)]
use std;
use std::cell::RefCell;
use std::rc::Rc;

use log::*;
use gio::{
    ApplicationExt, ApplicationExtManual, ApplicationFlags,
};
use glib;
use gtk;
use gtk::prelude::*;

use relm::init as relm_init;
use relm::Component;

use enamel_core::settings::Settings;
use enamel_core::database::Manager as DBManager;

use crate::constants;
use crate::main_window::MainWindow;
use crate::clone;
use crate::static_resource::new_builder;

#[derive(Debug, Clone)]
pub enum Action {
    RefreshAllViews,
    RefreshEpisodesView,
    RefreshEpisodesViewBGR,
    RefreshShowsView,
    // ReplaceWidget(Arc<Show>),
    RefreshWidgetIfSame(i32),
    // ShowWidgetAnimated,
    // ShowShowsAnimated,
    HeaderBarShowTile(String),
    HeaderBarNormal,
    HeaderBarShowUpdateIndicator,
    HeaderBarHideUpdateIndicator,
    // MarkAllPlayerNotification(Arc<Show>),
    // RemoveShow(Arc<Show>),
    // ErrorNotification(String),
    // InitEpisode(i32),
}

#[derive(Clone)]
pub struct EnamelApp {
    pub instance: gtk::Application,
    pub builder: gtk::Builder,
    window: RefCell<Option<Component<MainWindow>>>,
    // overlay: gtk::Overlay,
    pub settings: Rc<Settings>,
    pub dbmanager: Rc<DBManager>,

    // gio_settings: gio::Settings,
    // content: Rc<Content>,
    // headerbar: Rc<Header>,
    // player: Rc<player::PlayerWidget>,
    // sender: Sender<Action>,
    // receiver: Receiver<Action>,
}

impl EnamelApp {
    pub(crate) fn new(application: &gtk::Application,
                      settings: Rc<Settings>) -> Rc<Self> {
        // let settings = gio::Settings::new("com.github.vhdirk.Enamel");

        // let (sender, receiver) = unbounded();

        let builder = new_builder().unwrap();
        let dbmanager = Rc::new(DBManager::new(&settings));


        //let weak_s = settings.downgrade();
        // let weak_app = application.downgrade();
        // window.connect_delete_event(move |window, _| {
        //     let app = match weak_app.upgrade() {
        //         Some(a) => a,
        //         None => return Inhibit(false),
        //     };

        //     // let settings = match weak_s.upgrade() {
        //     //     Some(s) => s,
        //     //     None => return Inhibit(false),
        //     // };

        //     info!("Saving window position");
        //     //WindowGeometry::from_window(&window).write(&settings);

        //     info!("Application is exiting");
        //     app.quit();
        //     Inhibit(false)
        // });
        

        // let window = gtk::ApplicationWindow::new(application);
        // window.set_title(constants::APPLICATION_NAME);
        // window.set_wmclass(constants::APPLICATION_CLASS, constants::APPLICATION_NAME);
        // window.set_role(constants::APPLICATION_CLASS);
        // window.set_default_size(800, 600);
        
        // window.connect_delete_event(clone!(application, settings => move |window, _| {
        //     // WindowGeometry::from_window(&window).write(&settings);
        //     application.quit();
        //     Inhibit(false)
        // }));

        // Create a content instance
        // let content = Content::new(&sender).expect("Content Initialization failed.");

        // // Create the headerbar
        // let header = Header::new(&content, &sender);
        // // Add the Headerbar to the window.
        // window.set_titlebar(&header.container);

        // // Add the content main stack to the overlay.
        // let overlay = gtk::Overlay::new();
        // overlay.add(&content.get_stack());

        // let wrap = gtk::Box::new(gtk::Orientation::Vertical, 0);
        // // Add the overlay to the main Box
        // wrap.add(&overlay);

        // let player = player::PlayerWidget::new(&sender);
        // // Add the player to the main Box
        // wrap.add(&player.action_bar);

        // let window

        //window.add(&wrap);

        let app = EnamelApp {
            instance: application.clone(),
            settings,
            window: RefCell::new(None),
            builder,
            dbmanager
            // overlay,
            // headerbar: header,
            // content,
            // player,
            // sender,
            // receiver,
        };

        Rc::new(app)
    }

    fn init(app: &Rc<Self>) {
        // let cleanup_date = settings::get_cleanup_date(&app.settings);
        // Garbage collect watched episodes from the disk
        // utils::cleanup(cleanup_date);

        let window = relm_init::<MainWindow>(app.clone()).ok();
        app.window.replace(window);
        

        app.setup_gactions();
        app.setup_timed_callbacks();

        app.instance.connect_activate(clone!(app => move |_| app.activate()));


        // Retrieve the previous window position and size.
        // WindowGeometry::from_settings(&app.settings).apply(&app.window);

        // Setup the Action channel
        //gtk::timeout_add(25, clone!(app => move || app.setup_action_channel()));
    }

    pub fn activate(&self) {
        // TODO: broadcast activate signal
        let window: gtk::Window = self.builder.get_object("main_window")
                                              .expect("Couldn't find main_window in ui file.");
        window.show();
        window.present();
    }


    fn setup_timed_callbacks(&self) {
        // self.setup_dark_theme();
        // self.setup_refresh_on_startup();
        // self.setup_auto_refresh();
    }

    // fn setup_dark_theme(&self) {
    //     let gtk_settings = gtk::Settings::get_default().unwrap();
    //     let enabled = self.settings.get_boolean("dark-theme");
    //
    //     gtk_settings.set_property_gtk_application_prefer_dark_theme(enabled);
    // }

    // fn setup_refresh_on_startup(&self) {
    //     // Update the feeds right after the Application is initialized.
    //     let sender = self.sender.clone();
    //     if self.settings.get_boolean("refresh-on-startup") {
    //         info!("Refresh on startup.");
    //         // The ui loads async, after initialization
    //         // so we need to delay this a bit so it won't block
    //         // requests that will come from loading the gui on startup.
    //         gtk::timeout_add(1500, move || {
    //             let s: Option<Vec<_>> = None;
    //             utils::refresh(s, sender.clone());
    //             glib::Continue(false)
    //         });
    //     }
    // }

    // fn setup_auto_refresh(&self) {
    //     let refresh_interval = settings::get_refresh_interval(&self.settings).num_seconds() as u32;
    //     info!("Auto-refresh every {:?} seconds.", refresh_interval);
    //
    //     let sender = self.sender.clone();
    //     gtk::timeout_add_seconds(refresh_interval, move || {
    //         let s: Option<Vec<_>> = None;
    //         utils::refresh(s, sender.clone());
    //
    //         glib::Continue(true)
    //     });
    // }

    /// Define the `GAction`s.
    ///
    /// Used in menus and the keyboard shortcuts dialog.
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn setup_gactions(&self) {
        //let sender = &self.sender;
        let _win = &self.window;
        let _instance = &self.instance;
        // let header = &self.headerbar;

        // Create the `refresh` action.
        //
        // This will trigger a refresh of all the shows in the database.
        // action!(win, "refresh", clone!(sender => move |_, _| {
        //     gtk::idle_add(clone!(sender => move || {
        //         let s: Option<Vec<_>> = None;
        //         utils::refresh(s, sender.clone());
        //         glib::Continue(false)
        //     }));
        // }));
        // self.instance.set_accels_for_action("win.refresh", &["<primary>r"]);

        // Create the `OPML` import action
        // action!(win, "import", clone!(sender, win => move |_, _| {
        //     utils::on_import_clicked(&win, &sender)
        // }));

        // Create the action that shows a `gtk::AboutDialog`
        // action!(win, "about", clone!(win => move |_, _| about_dialog(&win)));

        // Create the quit action
        //action!(win, "quit", clone!(instance => move |_, _| instance.quit()));
        
        self.instance.set_accels_for_action("win.quit", &["<primary>q"]);

        // Create the menu action
        // action!(win, "menu",clone!(header => move |_, _| header.open_menu()));
        // Bind the hamburger menu button to `F10`
        self.instance.set_accels_for_action("win.menu", &["F10"]);
    }

    fn setup_action_channel(&self) -> glib::Continue {
        // if let Some(action) = self.receiver.try_recv() {
        //     trace!("Incoming channel action: {:?}", action);
        //     match action {
        //         // Action::RefreshAllViews => self.content.update(),
        //         // Action::RefreshShowsView => self.content.update_shows_view(),
        //         // Action::RefreshWidgetIfSame(id) => self.content.update_widget_if_same(id),
        //         // Action::RefreshEpisodesView => self.content.update_home(),
        //         // Action::RefreshEpisodesViewBGR => self.content.update_home_if_background(),
        //         // Action::ReplaceWidget(pd) => {
        //         //     let shows = self.content.get_shows();
        //         //     let mut pop = shows.borrow().populated();
        //         //     pop.borrow_mut()
        //         //         .replace_widget(pd.clone())
        //         //         .map_err(|err| error!("Failed to update ShowWidget: {}", err))
        //         //         .map_err(|_| error!("Failed ot update ShowWidget {}", pd.title()))
        //         //         .ok();
        //         // }
        //         // Action::ShowWidgetAnimated => {
        //         //     let shows = self.content.get_shows();
        //         //     let mut pop = shows.borrow().populated();
        //         //     pop.borrow_mut().switch_visible(
        //         //         PopulatedState::Widget,
        //         //         gtk::StackTransitionType::SlideLeft,
        //         //     );
        //         // }
        //         // Action::ShowShowsAnimated => {
        //         //     let shows = self.content.get_shows();
        //         //     let mut pop = shows.borrow().populated();
        //         //     pop.borrow_mut()
        //         //         .switch_visible(PopulatedState::View, gtk::StackTransitionType::SlideRight);
        //         // }
        //         // Action::HeaderBarShowTile(title) => self.headerbar.switch_to_back(&title),
        //         // Action::HeaderBarNormal => self.headerbar.switch_to_normal(),
        //         // Action::HeaderBarShowUpdateIndicator => self.headerbar.show_update_notification(),
        //         // Action::HeaderBarHideUpdateIndicator => self.headerbar.hide_update_notification(),
        //         // Action::MarkAllPlayerNotification(pd) => {
        //         //     let notif = mark_all_notif(pd, &self.sender);
        //         //     notif.show(&self.overlay);
        //         // }
        //         // Action::RemoveShow(pd) => {
        //         //     let notif = remove_show_notif(pd, self.sender.clone());
        //         //     notif.show(&self.overlay);
        //         // }
        //         // Action::ErrorNotification(err) => {
        //         //     error!("An error notification was triggered: {}", err);
        //         //     let callback = || glib::Continue(false);
        //         //     let notif = InAppNotification::new(&err, callback, || {}, UndoState::Hidden);
        //         //     notif.show(&self.overlay);
        //         // }
        //         // Action::InitEpisode(rowid) => self.player.initialize_episode(rowid).unwrap(),
        //         _ => ()
        //     }
        // }

        glib::Continue(true)
    }

    pub fn run(settings: Rc<Settings>) {
        let application = gtk::Application::new(constants::APPLICATION_ID, ApplicationFlags::empty())
            .expect("Application Initialization failed...");

        application.set_resource_base_path("/com/github/vhdirk/Enamel");

        let weak_app = application.downgrade();
        application.connect_startup(move |_| {
            info!("GApplication::startup");
            weak_app.upgrade().map(|application| {
                let mut app = Self::new(&application, settings.clone());
                Self::init(&mut app);
                info!("Init complete");
            });
        });

        // Weird magic I copy-pasted that sets the Application Name in the Shell.
        glib::set_application_name(constants::APPLICATION_NAME);
        glib::set_prgname(Some(constants::APPLICATION_NAME));

        // We need out own Enamel icon
        gtk::Window::set_default_icon_name(constants::APPLICATION_ICON_NAME);

        // Run GTK application with command line args
        let args: Vec<String> = std::env::args().collect();
        ApplicationExtManual::run(&application, &args);
    }
}


