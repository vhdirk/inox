use inox_core::models::query::Query;

#[derive(Debug)]
pub enum Action {
    SelectTag(Option<String>),
    Search(String),
    Query(Query),
    SelectConversation(Option<String>),
    SelectConversations(Vec<String>),

    // Reload,
    // ViewShowLibrary,
    // ViewShowPlayer,
    // ViewRaise,
    // ViewShowNotification(Rc<Notification>),
    // PlaybackConnectGCastDevice(GCastDevice),
    // PlaybackDisconnectGCastDevice,
    // PlaybackSetStation(Box<Station>),
    // PlaybackStart,
    // PlaybackStop,
    // PlaybackSetVolume(f64),
    // PlaybackSaveSong(Song),
    // LibraryAddStations(Vec<Station>),
    // LibraryRemoveStations(Vec<Station>),
    // SearchFor(StationRequest), // TODO: is this neccessary?,
    // SettingsKeyChanged(Key)
}
