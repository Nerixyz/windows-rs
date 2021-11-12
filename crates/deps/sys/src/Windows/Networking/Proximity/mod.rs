#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceArrivedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceDepartedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerFinderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerFinderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerInformation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerInformationWithHostAndService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeerWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximityDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximityDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximityMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITriggeredConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageReceivedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageTransmittedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeerDiscoveryTypes(pub u32);
impl PeerDiscoveryTypes {
    pub const None: Self = Self(0u32);
    pub const Browse: Self = Self(1u32);
    pub const Triggered: Self = Self(2u32);
}
impl ::core::marker::Copy for PeerDiscoveryTypes {}
impl ::core::clone::Clone for PeerDiscoveryTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PeerInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeerRole(pub i32);
impl PeerRole {
    pub const Peer: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Client: Self = Self(2i32);
}
impl ::core::marker::Copy for PeerRole {}
impl ::core::clone::Clone for PeerRole {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PeerWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeerWatcherStatus(pub i32);
impl PeerWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for PeerWatcherStatus {}
impl ::core::clone::Clone for PeerWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProximityDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProximityMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TriggeredConnectState(pub i32);
impl TriggeredConnectState {
    pub const PeerFound: Self = Self(0i32);
    pub const Listening: Self = Self(1i32);
    pub const Connecting: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const Failed: Self = Self(5i32);
}
impl ::core::marker::Copy for TriggeredConnectState {}
impl ::core::clone::Clone for TriggeredConnectState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TriggeredConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);