use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Contact {
    pub name: Option<String>,
    pub email_address: String, //TODO: need a better type here
}

impl Contact {
    pub fn is_spoofed(&self) -> bool {
        // Empty test and regexes must apply to the raw values, not
        // clean ones, otherwise any control chars present will have
        // been lost

        let is_spoof = false;

        // 1. Check the name part contains no controls and doesn't
        // look like an email address (unless it's the same as the
        // address part).
        if let Some(name) = self.name.as_ref() {
            if !name.is_empty() {
            // if (Regex.match_simple(CONTROLS, this.name)) {
            //     is_spoof = true;
            // } else if (has_distinct_name()) {
            //     // Clean up the name as usual, but remove all
            //     // whitespace so an attack can't get away with a name
            //     // like "potus @ whitehouse . gov"
            //     string clean_name = Geary.String.reduce_whitespace(this.name);
            //     clean_name = clean_name.replace(" ", "");
            //     if (is_valid_address(clean_name)) {
            //         is_spoof = true;
            //     }
            // }
            }
        }

        // 2. Check the mailbox part of the address doesn't contain an
        // @. Is actually legal if quoted, but rarely (never?) found
        // in the wild and better be safe than sorry.
        // if let Some(email_address) = self.dynamic_cast_ref::<gmime::InternetAddressMailbox>() {
        //     if let Some(addr) = mailbox_address.addr() {
        //         if !is_spoof && addr.contains('@') {
        //             return true;
        //         }
        //     }
        // }


        // 3. Check the address doesn't contain any spaces or
        // controls. Again, space in the mailbox is allowed if quoted,
        // but in practice should rarely be used.
        // if (!is_spoof && Regex.match_simple(Geary.String.WS_OR_NP, this.address)) {
        //     is_spoof = true;
        // }

        return is_spoof;
    }
}