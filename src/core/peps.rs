use byteorder::{LittleEndian, WriteBytesExt};

use crate::model::{data::Message, unicode::Unicode};

pub(crate) struct SessionSelectionPDU {
    version: Version,
}

enum Version {
    RdpPreconnectionPduV1 { id: u32 },
    RdpPreconnectionPduV2 { id: String },
}

impl SessionSelectionPDU {
    pub fn with_id(id: u32) -> SessionSelectionPDU {
        SessionSelectionPDU {
            version: Version::RdpPreconnectionPduV1 { id },
        }
    }

    pub fn with_unicode_id(id: &str) -> SessionSelectionPDU {
        SessionSelectionPDU {
            version: Version::RdpPreconnectionPduV2 {
                id: String::from(id),
            },
        }
    }
}

impl Message for SessionSelectionPDU {
    fn write(&self, writer: &mut dyn std::io::Write) -> crate::model::error::RdpResult<()> {
        match &self.version {
            Version::RdpPreconnectionPduV1 { id } => {
                writer.write_u32::<LittleEndian>(16u32)?;
                writer.write_u32::<LittleEndian>(0)?;
                writer.write_u32::<LittleEndian>(1u32)?;
                writer.write_u32::<LittleEndian>(*id)?;
            }
            Version::RdpPreconnectionPduV2 { id } => {
                let blob = Unicode::to_unicode(id);
                let size = 18u32 + blob.len() as u32 + 2u32;

                writer.write_u32::<LittleEndian>(size)?;
                writer.write_u32::<LittleEndian>(0)?;
                writer.write_u32::<LittleEndian>(2u32)?;
                writer.write_u32::<LittleEndian>(0u32)?;
                writer.write_u16::<LittleEndian>(id.len() as u16 + 1u16)?;
                writer.write_all(&blob)?;
                writer.write_u16::<LittleEndian>(0u16)?;
            }
        };

        Ok(())
    }

    fn read(&mut self, reader: &mut dyn std::io::Read) -> crate::model::error::RdpResult<()> {
        todo!()
    }

    fn length(&self) -> u64 {
        todo!()
    }

    fn visit(&self) -> crate::model::data::DataType {
        todo!()
    }

    fn options(&self) -> crate::model::data::MessageOption {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::SessionSelectionPDU;
    use crate::model::data::Message;
    use hex;
    #[test]
    fn sample_v1_encoding() {
        let id: u32 = 0xEEC699EB;

        let pdu = SessionSelectionPDU::with_id(id);
        let mut write = std::io::Cursor::new(Vec::<u8>::new());
        pdu.write(&mut write).unwrap();
        let actual = write.into_inner();

        let expected = hex::decode("100000000000000001000000eb99c6ee").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample_v2_encoding() {
        let blob = "TestVM";
        let pdu = SessionSelectionPDU::with_unicode_id(blob);
        let mut write = std::io::Cursor::new(Vec::<u8>::new());
        pdu.write(&mut write).unwrap();
        let actual = write.into_inner();

        let expected =
            hex::decode("200000000000000002000000000000000700540065007300740056004d000000")
                .unwrap();
        assert_eq!(expected, actual);
    }
}
