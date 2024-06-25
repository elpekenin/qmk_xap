use core::fmt::Debug;
use std::io::Cursor;

use binrw::{binread, BinRead, BinReaderExt, NullString};
use log::trace;

use crate::error::XAPResult;
use crate::token::Token;

use crate::protocol::xap::XAPSecureStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
#[binread]
#[br(repr = u8)]
pub enum BroadcastType {
    Log = 0,
    SecureStatus = 1,
    Keyboard = 2,
    User = 3,
}

#[binread]
#[derive(Debug)]
pub struct BroadcastRaw {
    _token: Token,
    broadcast_type: BroadcastType,
    #[br(temp)]
    payload_len: u8,
    #[br(count = payload_len)]
    payload: Vec<u8>,
}

impl BroadcastRaw {
    #[must_use]
    pub fn broadcast_type(&self) -> &BroadcastType {
        &self.broadcast_type
    }

    pub fn from_raw_report(report: &[u8]) -> XAPResult<Self> {
        let mut reader = Cursor::new(report);
        let broadcast = Self::read_le(&mut reader)?;
        trace!("received raw XAP broadcast: {:#?}", broadcast);
        Ok(broadcast)
    }

    pub fn into_xap_broadcast<T>(self) -> XAPResult<T>
    where
        T: XAPBroadcast,
    {
        let mut reader = Cursor::new(&self.payload);
        Ok(T::read_le(&mut reader)?)
    }
}

pub trait XAPBroadcast: Sized + Debug + BinRead<Args = ()> {}

#[derive(Debug)]
pub struct LogBroadcast(pub String);

impl BinRead for LogBroadcast {
    type Args = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _options: &binrw::ReadOptions,
        _args: Self::Args,
    ) -> binrw::BinResult<Self> {
        Ok(Self(std::io::read_to_string(reader)?))
    }
}

impl XAPBroadcast for LogBroadcast {}

#[derive(BinRead, Debug)]
pub struct SecureStatusBroadcast(pub XAPSecureStatus);

impl XAPBroadcast for SecureStatusBroadcast {}

// ===============
// Custom messages
#[derive(BinRead, Debug, Clone)]
pub struct ScreenPressed {
    pub screen_id: u8,
    pub x: u16,
    pub y: u16,
}
impl XAPBroadcast for ScreenPressed {}

#[derive(BinRead, Debug, Clone)]
pub struct ScreenReleased {
    pub screen_id: u8,
}
impl XAPBroadcast for ScreenReleased {}

#[derive(BinRead, Debug, Clone)]
pub struct LayerChanged {
    pub layer: u8,
}
impl XAPBroadcast for LayerChanged {}

#[derive(BinRead, Debug, Clone)]
pub struct KeyEvent {
    pub keycode: u16,
    pub pressed: u8,
    pub layer: u8,
    pub row: u8,
    pub col: u8,
    pub mods: u8,
    pub str: NullString,
}
impl XAPBroadcast for KeyEvent {}

#[derive(BinRead, Debug, Clone)]
pub struct Shutdown {
    pub bootloader: u8,
}
impl XAPBroadcast for Shutdown {}

#[derive(BinRead, Debug, Clone)]
pub struct KeyTester {
    pub pressed: u8,
    pub row: u8,
    pub col: u8,
}
impl XAPBroadcast for KeyTester {}

// - Aggregate
#[derive(BinRead, Debug, Clone)]
pub enum UserBroadcast {
    #[br(magic = 0u8)]
    ScreenPressed(ScreenPressed),

    #[br(magic = 1u8)]
    ScreenReleased(ScreenReleased),

    #[br(magic = 2u8)]
    LayerChanged(LayerChanged),

    #[br(magic = 3u8)]
    KeyEvent(KeyEvent),

    #[br(magic = 4u8)]
    Shutdown(Shutdown),

    #[br(magic = 5u8)]
    KeyTester(KeyTester),
}

impl XAPBroadcast for UserBroadcast {}
