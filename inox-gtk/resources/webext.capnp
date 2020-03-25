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

    clearMessages @2() -> ();
    addMessages @3 (messages: List(Message));
}


struct Address {
    name @0: Text;
    email @1: Text;
    fullAddress @2: Text;
}

struct Chunk {
    struct Signature {
        verified @0: Bool;

        signStrings @1: List(Text);
        allErrors @2: List(Text);
    }

    struct Encryption {
        decrypted @0: Bool;
        encStrings @1: List(Text);
    }

    id @0: UInt32;
    sid @1: Text;
    mimeType @2: Text;
    cid @3: Text;

    viewable @4: Bool;
    preferred @5: Bool;
    attachment @6: Bool;

    encrypted @7: Bool;
    signed @8: Bool;

    cryptoId @9: Int32;

    signature @10: Signature;
    encryption @11: Encryption;

    sibling @12: Bool;
    used @13: Bool;
    focusable @14: Bool;

    content @15: Text;
    filename @16: Text;
    size @17: UInt32;
    humanSize @18: Text;

    thumbnail @19: Text; # used by attachments

    children @20: List(Chunk);
    siblings @21: List(Chunk);
}

struct Message {
    id @0: Text;
    sender @1: Address;
    to @2: List(Address);
    cc @3: List(Address);
    bcc @4: List(Address);
    replyTo @5: List(Address);

    datePretty @6: Text;
    dateVerbose @7: Text;

    subject @8: Text;
    tags @9: List(Text);
    tagString @10: Text;

    gravatar @11: Text;

    missingContent @12: Bool;
    patch @13: Bool;
    differentSubject @14: Bool;

    level @15: Int32;
    inReplyTo @16: Text;

    preview @17: Text;

    root @18: Chunk;

    mimeMessages @19: List(Chunk);
    attachments @20: List(Chunk);
}


