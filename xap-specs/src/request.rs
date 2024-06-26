use core::fmt::Debug;
use std::io::{Seek, Write};

use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};

use crate::token::Token;

pub trait XapRequest: Sized + Debug + for<'a> BinWrite<Args<'a> = ()> {
    type Response: for<'a> BinRead<Args<'a> = ()>;

    fn id() -> &'static [u8];

    fn xap_version() -> u32;

    fn is_secure() -> bool {
        false
    }
}

pub struct RawRequest<T: XapRequest> {
    token: Token,
    payload: T,
}

impl<T> RawRequest<T>
where
    T: XapRequest,
{
    pub fn new(payload: T) -> Self {
        Self {
            token: Token::regular_token(),
            payload,
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl<T> BinWrite for RawRequest<T>
where
    T: XapRequest,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        writer.write_le(&self.token)?;
        // Dummy write of the payload length, which is not known at this point.
        writer.write_le(&0_u8)?;
        writer.write_le(&T::id())?;
        writer.write_le(&self.payload)?;

        // Calculate payload size from current position in the writer stream,
        // which points at the end of payload and contains the Token and payload
        // lenght field itself. These have to be substracted to get the total
        // size of the payload.
        let payload_length = writer.stream_position()?
            - std::mem::size_of::<u16>() as u64 // Token
            - std::mem::size_of::<u8>() as u64; // payload length field

        // Position our writer on the payload_length field again and write the correct value.
        writer.seek(std::io::SeekFrom::Start(2))?;
        writer.write_le(&(payload_length as u8))
    }
}
