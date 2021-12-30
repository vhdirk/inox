use gmime;
use glib::IsA;
use gmime::{InternetAddressExt, InternetAddressMailboxExt};
use glib::Cast;
const CONTROLS: &str = "[[:cntrl:]]+";


pub trait InternetAddressAux {
    /**
     * Determines if the mailbox address appears to have been spoofed.
     *
     * Using recipient and sender mailbox addresses where the name
     * part is also actually a valid RFC822 address
     * (e.g. "you@example.com <jerk@spammer.com>") is a common tactic
     * used by spammers and malware authors to exploit MUAs that will
     * display the name part only if present. It also enables more
     * sophisticated attacks such as
     * [[https://www.mailsploit.com/|Mailsploit]], which uses
     * Quoted-Printable or Base64 encoded nulls, new lines, @'s and
     * other characters to further trick MUAs into displaying a bogus
     * address.
     *
     * This method attempts to detect such attacks by examining the
     * {@link name} for non-printing characters and determining if it
     * is by itself also a valid RFC822 address.
     *
     * @return //true// if the complete decoded address contains any
     * non-printing characters, if the name part is also a valid
     * RFC822 address, or if the address part is not a valid RFC822
     * address.
     */
    fn is_spoofed(&self) -> bool;
}

impl<O: IsA<gmime::InternetAddress>> InternetAddressAux for O  {

    fn is_spoofed(&self) -> bool {
        // Empty test and regexes must apply to the raw values, not
        // clean ones, otherwise any control chars present will have
        // been lost

        let is_spoof = false;

        // 1. Check the name part contains no controls and doesn't
        // look like an email address (unless it's the same as the
        // address part).
        if let Some(name) = self.name() {
            if !name.to_string().is_empty() {
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
        if let Some(mailbox_address) = self.dynamic_cast_ref::<gmime::InternetAddressMailbox>() {
            if let Some(addr) = mailbox_address.addr() {
                if !is_spoof && addr.contains('@') {
                    return true;
                }
            }
        }


        // 3. Check the address doesn't contain any spaces or
        // controls. Again, space in the mailbox is allowed if quoted,
        // but in practice should rarely be used.
        // if (!is_spoof && Regex.match_simple(Geary.String.WS_OR_NP, this.address)) {
        //     is_spoof = true;
        // }

        return is_spoof;
    }
}

