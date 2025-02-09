use ipld_core::cid::{multibase::Base, Cid};
use serde::{de, Deserialize, Serialize};

/// Result of deserializing a DAG-JSON map consisting of the reserved key `/`.
///
/// The values are the already parsed/decoded data.
#[derive(Debug)]
pub(crate) enum ReservedKeyValueParsed {
    Link(Cid),
    Bytes(Vec<u8>),
}

/// Used for deserializing a DAG-JSON map, consisting of the reserved key `$link`.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ReservedKeyLinkMap {
    #[serde(rename = "$link")]
    pub(crate) _link: String,
}

/// Used for deserializing a DAG-JSON map, consisting of the reserved key `$bytes`.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ReservedKeyBytesMap {
    #[serde(rename = "$bytes")]
    pub(crate) _bytes: String,
}

impl ReservedKeyLinkMap {
    pub(crate) fn parse<E>(&self) -> Result<ReservedKeyValueParsed, E>
    where
        E: de::Error,
    {
        let cid = Cid::try_from(&self._link[..])
            .map_err(|_| de::Error::custom(format!("Invalid CID `{}`", self._link)))?;
        Ok(ReservedKeyValueParsed::Link(cid))
    }
}

impl ReservedKeyBytesMap {
    pub(crate) fn parse<E>(&self) -> Result<ReservedKeyValueParsed, E>
    where
        E: de::Error,
    {
        let bytes = Base::Base64.decode(&self._bytes[..]).map_err(|_| {
            de::Error::custom(format!("Cannot base decode bytes `{}`", self._bytes))
        })?;
        Ok(ReservedKeyValueParsed::Bytes(bytes))
    }
}
