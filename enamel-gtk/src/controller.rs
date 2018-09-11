use gtk;

use uibuilder;


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


pub struct EnamelController {
    pub ui: uibuilder::UI,
    pub gtk_app: gtk::Application,
    //pub backend: Sender<backend::BKCommand>,
    //pub internal: Sender<InternalCommand>,
}


impl EnamelController {
    pub fn new(app: gtk::Application,
               ui: uibuilder::UI
            //    tx: Sender<BKCommand>,
            //    itx: Sender<InternalCommand>
            ) -> EnamelController {
        EnamelController {
            ui: ui,
            gtk_app: app,
        }
    }
}