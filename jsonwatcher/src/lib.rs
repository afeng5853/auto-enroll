extern crate reqwest;
extern crate failure;
extern crate blake2;

use std::collections::HashMap;
use failure::Error;
use blake2::{Blake2b, Digest};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

/// Gets JSON from GET request on URL and tracks changes
pub struct Requester<'a> {
    client: reqwest::Client,
    /// Maps JSON Key in track_keys -> hash(JSON value)
    json_hash : HashMap<String, Vec<u8>>,
    /// URL to track
    url: &'a str,
    /// GET params
    params: &'a[(&'a str, &'a str)],
    /// Track specific root JSON keys. TODO: If left empty, tracks all and deal with arrays and recursive keys
    track_keys: &'a [&'a str]
}

impl<'a> Requester<'a> {
    /// Creates a new Requester
    ///
    /// # Arguments
    /// * `url` - A string slice of the URL to track
    /// * `params` Array reference containing two-tuples representing GET params
    /// * `track_keys` Array reference of string slices containing root JSON keys to be tracked
    ///
    /// # Example
    /// ```
    /// use autoenroll::Requester;
    /// let requester = Requester::new("https://jsonplaceholder.typicode.com/todos/1", &[], &["title"]);
    /// ```
    pub fn new(url: &'a str, params: &'a [(&str, &str)], track_keys: &'a [&'a str]) -> Self {
        Requester {
            client: reqwest::Client::new(),
            json_hash : HashMap::new(),
            track_keys,
            url,
            params
        }
    }

    /// Requests JSON through GET request on URL without any side effects (tracking changes)
    ///
    /// # Example
    /// ```
    /// use autoenroll::Requester;
    /// let mut requester = Requester::new("https://jsonplaceholder.typicode.com/todos/1", &[], &["title"]);
    /// dbg!(requester.get_json());
    /// ```
    pub fn get_json(&mut self) -> Result<HashMap<String, String>, Error> {
        let mut resp = self.client.get(self.url)
            .query(self.params)
            .send()?;
        let json = resp.json();
        Ok(json?)
    }

    /// Calls get_json, tracks values of tracked keys, and returns changes
    ///
    /// # Example
    /// ```
    /// use autoenroll::Requester;
    /// let mut requester = Requester::new("https://jsonplaceholder.typicode.com/todos/1", &[], &["title"]);
    /// dbg!(requester.get_changes()); // Always returns JSON on first call when a new requester is made
    ///                                // because no changes were tracked previously
    /// dbg!(requester.get_changes());
    /// ```
    pub fn get_changes(&mut self) -> Result<HashMap<String, String>, Error> {
        let json = self.get_json()?;

        // return detected changes as JSON
        let mut return_changes : HashMap<String, String> = HashMap::new();

        // iterate through each key to detect changes in the corresponding value
        for key in self.track_keys.iter() {
            let key = key.to_string();
            let val = json.get(&key); // detect changes in this variable
            let prev_hash = self.json_hash.get(&key); // previous hash to compare to

            if let Some(s) = val {
                let mut hasher = Blake2b::new();
                hasher.input(s);
                let cur_hash = hasher.result().to_vec();

                match prev_hash {
                    Some(prev_hash) => {
                        if !cur_hash.eq(prev_hash) {
                            return_changes.insert(key.clone(), s.clone());
                            self.json_hash.insert(key, cur_hash);
                        }
                    },
                    None => {
                        return_changes.insert(key.clone(), s.clone());
                        self.json_hash.insert(key, cur_hash);
                    }
                }
            } else {
                println!("Key not found: {}", key);
            }
        }
        Ok(return_changes)
    }
}

/// Event handler for tracking JSON changes
///
/// # Arguments
/// * `req` - A Requester that outputs changes for detection
/// * `check_every` - A duration that specifies how long to wait until checking for the next change
/// * `callback` - Callback function that takes in a JSON HashMap<String, String>
pub fn on_change<A>(mut req : Requester<'static>, check_every : Duration, callback: impl Fn(HashMap<String, String>) -> A + Send + 'static)
                -> JoinHandle<()> {
    spawn(move || {
        loop {
            let changes = req.get_changes().unwrap();
            if !changes.is_empty() {
                callback(changes);
            }
            sleep(check_every);
        }
    })
}