/* 
 * screencat - use sendcat.com to share screenshots automatically
 * Copyright (C) 2018  Josh Lindsey <joshua.s.lindsey@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use ::errors::*;

use ::std::borrow::Borrow;
use ::std::cell::RefCell;
use ::std::env;
use ::std::fs;
use ::std::path::Path;
use ::std::process::{Command, Stdio};
use ::std::time::Duration;
use ::std::sync::mpsc::{channel, Receiver};
use ::clipboard::{ClipboardProvider, ClipboardContext};
use ::notify::{DebouncedEvent, Watcher, RecommendedWatcher, RecursiveMode, watcher};
use ::shellexpand;
use ::rand::{Rng, thread_rng};
use ::regex;
use ::serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct SendcatShareFiles {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SendcatOutput {
    share_files: Vec<SendcatShareFiles>,
}

pub struct FSWatcher {
    pub rx: Receiver<DebouncedEvent>,
    watched_path: String,
    watcher: RefCell<RecommendedWatcher>,
    re: regex::Regex,
    clipboard_ctx: RefCell<ClipboardContext>,
}

impl FSWatcher {
    pub fn new(watch_path: &str, regex_str: &str, delay: u64) -> Result<Self> {
        let (tx, rx) = channel::<DebouncedEvent>();
        let watcher = watcher(tx, Duration::from_millis(delay))?;
        let real_path = shellexpand::tilde(watch_path);
        let ctx: ClipboardContext = ClipboardProvider::new().expect("unable to initialize clipboard context");

        let re = regex::Regex::new(regex_str)?;

        Ok(FSWatcher{
            rx: rx,
            watched_path: real_path.into_owned(),
            watcher: RefCell::new(watcher),
            re: re,
            clipboard_ctx: RefCell::new(ctx),
        })
    }

    pub fn start(&self) -> Result<()> {
        info!("Starting watcher at {}", self.watched_path);
        let mut watcher = self.watcher.borrow_mut();
        watcher.watch(&self.watched_path, RecursiveMode::NonRecursive)?;

        loop {
            match self.rx.recv() {
                Ok(DebouncedEvent::Create(ref create_path)) | Ok(DebouncedEvent::Chmod(ref create_path)) => {
                    let str_path = create_path.to_string_lossy();
                    self.handle_event(str_path.borrow())?;
                }
                Ok(event) => debug!("ignoring event: {:?}", event),
                Err(e) => bail!(e),
            }
        }
    }

    fn handle_event(&self, path: &str) -> Result<()> {
        if !self.re.is_match(path) {
            debug!("path does not match regex: {}", path);
            return Ok(());
        }

        if !Path::new(path).exists() {
            warn!("path doesn't exist after all: {}", path);
            return Ok(())
        }

        info!("New screenshot: {}", path);

        let mut new_path = env::temp_dir();
        new_path.push(self.rand_file_name());
        debug!("copying to tmp file {:?}", new_path);
        fs::copy(path, &new_path)?;

        debug!("running sendcat");
        let output = Command::new("sendcat")
                             .arg("-j")
                             .arg(&new_path)
                             .stdin(Stdio::inherit())
                             .output()
                             .chain_err(|| "error while running sendcat")?;

        debug!("sendcat output: {:?}", output);

        let out: SendcatOutput = serde_json::from_slice(&output.stdout)?;
        let url = &out.share_files[0].url;
        debug!("parsed URL from output: {}", url);

        self.copy_url_to_clipboard(url)?;
        if cfg!(target_os = "macos") {
            self.show_os_notification(url)?;
        }

        debug!("removing original screenshot: {}", path);
        fs::remove_file(path)?;

        debug!("removing tmp file: {:?}", new_path);
        fs::remove_file(new_path)?;

        Ok(())
    }

    fn rand_file_name(&self) -> String {
        thread_rng().gen_ascii_chars().take(10).collect()
    }

    fn copy_url_to_clipboard(&self, url: &str) -> Result<()> {
        let mut ctx = self.clipboard_ctx.borrow_mut();
        if let Err(err) = ctx.set_contents(url.to_owned()) {
            bail!("unable to set clipboard contents: {}", err)
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn show_os_notification(&self, url: &str) -> Result<()> {
        let cmd = Command::new("terminal-notifier")
                          .args(&["-title", "Screencat"])
                          .args(&["-subtitle", "Screenshot Uploaded"])
                          .args(&["-message", url])
                          .args(&["-open", url])
                          .args(&["-group", "me.jlindsey.screencat"])
                          .status();

        match cmd {
            Ok(status) => {
                if !status.success() {
                    match status.code() {
                        Some(code) => warn!("unable to send os notification: {}", code),
                        None => warn!("unable to send os notification: unknown exit code")
                    }
                }

                Ok(())
            },
            Err(err) => bail!("error when sending os notification: {}", err)
        }
    }
}
