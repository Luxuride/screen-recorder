use super::portal::OrgFreedesktopPortalScreenCast;
use dbus::arg::{PropMap, Variant};
use dbus::blocking::Connection;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::os::fd::FromRawFd;
use std::time::Duration;

pub fn record() -> Result<(), Box<dyn Error>> {
    let conn = Connection::new_session()?;

    let mut map = PropMap::new();
    let token = format!("test_{}", rand::random::<u16>());
    map.insert("handle_token".to_owned(), Variant(Box::new(token.clone())));
    map.insert(
        String::from("session_handle_token"),
        Variant(Box::new(token.clone())),
    );
    let proxy = conn.with_proxy("org.freedesktop.portal.Desktop", "/org/freedesktop/portal/desktop", Duration::from_millis(5000));
    let session = proxy.create_session(map)?;
    let mut map = PropMap::new();
    map.insert("handle_token".to_owned(), Variant(Box::new(token.clone())));
    map.insert("types".to_owned(), Variant(Box::new(proxy.available_source_types()?)));
    map.insert(String::from("multiple"), Variant(Box::new(false)));
    map.insert("cursor_mode".to_owned(), Variant(Box::new(proxy.available_cursor_modes()?)));
    let source = proxy.select_sources(session, map).unwrap();
    let start = proxy.start(source, "", PropMap::new())?;
    let remote = proxy.open_pipe_wire_remote(start, PropMap::new())?;
    let mut file = unsafe { File::from_raw_fd(remote.into_fd()) };
    let mut buff = [0_u8; 32];
    file.read(&mut buff)?;
    println!("{:?}", buff);
    Ok(())
}