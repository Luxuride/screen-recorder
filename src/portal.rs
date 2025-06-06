// This code was autogenerated with `dbus-codegen-rust -d org.freedesktop.portal.Desktop -p /org/freedesktop/portal/desktop -f org.freedesktop.portal.ScreenCast`, see https://github.com/diwic/dbus-rs
use dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopPortalScreenCast {
    fn create_session(&self, options: arg::PropMap) -> Result<dbus::Path<'static>, dbus::Error>;
    fn select_sources(
        &self,
        session_handle: dbus::Path,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn start(
        &self,
        session_handle: dbus::Path,
        parent_window: &str,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn open_pipe_wire_remote(
        &self,
        session_handle: dbus::Path,
        options: arg::PropMap,
    ) -> Result<arg::OwnedFd, dbus::Error>;
    fn available_source_types(&self) -> Result<u32, dbus::Error>;
    fn available_cursor_modes(&self) -> Result<u32, dbus::Error>;
    fn version(&self) -> Result<u32, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopPortalScreenCast for blocking::Proxy<'a, C>
{
    fn create_session(&self, options: arg::PropMap) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.ScreenCast",
            "CreateSession",
            (options,),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn select_sources(
        &self,
        session_handle: dbus::Path,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.ScreenCast",
            "SelectSources",
            (session_handle, options),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn start(
        &self,
        session_handle: dbus::Path,
        parent_window: &str,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.ScreenCast",
            "Start",
            (session_handle, parent_window, options),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn open_pipe_wire_remote(
        &self,
        session_handle: dbus::Path,
        options: arg::PropMap,
    ) -> Result<arg::OwnedFd, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.ScreenCast",
            "OpenPipeWireRemote",
            (session_handle, options),
        )
        .and_then(|r: (arg::OwnedFd,)| Ok(r.0))
    }

    fn available_source_types(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            self,
            "org.freedesktop.portal.ScreenCast",
            "AvailableSourceTypes",
        )
    }

    fn available_cursor_modes(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            self,
            "org.freedesktop.portal.ScreenCast",
            "AvailableCursorModes",
        )
    }

    fn version(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            self,
            "org.freedesktop.portal.ScreenCast",
            "version",
        )
    }
}
