extern crate winrt_notification;
use winrt_notification::{Duration, Sound, Toast};
use std::collections::HashMap;

pub fn desktop_notify(json : HashMap<String, String>) {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Change detected.")
        .text1("(╯°□°）╯︵ ┻━┻")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}