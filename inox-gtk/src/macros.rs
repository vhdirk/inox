/// Creates an action named $called in the action map $on with the handler $handle
#[macro_export]
macro_rules! action {
    ($on:expr, $called:expr, $handle:expr) => {{
        use gio::SimpleAction;
        // Create a stateless, parameterless action
        let act = SimpleAction::new($called, None);
        // Connect the handler
        act.connect_activate($handle);
        // Add it to the map
        $on.add_action(&act);
        // Return the action
        act
    }};
}

#[macro_export]
macro_rules! spawn {
    ($future:expr) => {
        let ctx = glib::MainContext::default();
        ctx.spawn_local($future);
    };
}
