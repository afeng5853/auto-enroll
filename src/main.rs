use failure::Error;
use std::time::Duration;
use jsonwatcher::{Requester, on_change};
use notify::desktop_notify;

fn main() -> Result<(), Error> {
    let requester = Requester::new("http://classfind.stonybrook.edu/vufind/AJAX/JSON?method=getItemVUStatuses",
                                       &[("itemid", "90280"), ("strm", "1198")],
                                        &["data"]);
    let thread = on_change(requester, Duration::from_secs(10), desktop_notify);

    thread.join().unwrap_err();
    Ok(())
}