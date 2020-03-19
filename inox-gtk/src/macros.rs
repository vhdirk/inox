// http://gtk-rs.org/tuto/closures
#[macro_export]
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

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
macro_rules! get_widget {
    ($builder:expr, $wtype:ty, @$name:ident) => {{
        let $name: $wtype = $builder.get_object(stringify!($name)).expect(&format!("Could not find widget \"{}\"", stringify!($name)));
        $name
    }};
    ($builder:expr, $wtype:ty, $name:ident) => {
        let $name: $wtype = $builder.get_object(stringify!($name)).expect(&format!("Could not find widget \"{}\"", stringify!($name)));
    };
}

#[macro_export]
macro_rules! spawn {
    ($future:expr) => {
        let ctx = glib::MainContext::default();
        ctx.spawn_local($future);
    };
}
