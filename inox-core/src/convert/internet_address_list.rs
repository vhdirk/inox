use gmime::InternetAddressListExt;
use gmime;
use glib::IsA;
use gmime::{InternetAddressExt, InternetAddressMailboxExt};
use glib::Cast;


pub trait InternetAddressListAux {

}

impl<O: IsA<gmime::InternetAddressList>> InternetAddressListAux for O  {

}