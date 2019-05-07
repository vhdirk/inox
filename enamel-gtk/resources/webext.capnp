@0xb8a81e0cebeae0d7;


interface Page{

    allowRemoteImages @0 (allow: Bool) -> ();
    load @1(html: Text,
            css: Text,
            partCss: Text,
            allowedUris: List(Text),
            useStdout: Bool,
            useSyslog: Bool,
            disableLog: Bool,
            logLevel: Text) -> ();




}


interface Message{

    # Mark(/*message_id:*/String,
    #      /*marked:*/bool)#
    # Hidden(/*message_id:*/String,
    #        /*hidden:*/bool),


}