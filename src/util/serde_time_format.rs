use chrono::{Local, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt_utc = date.and_local_timezone(Utc).unwrap();
    dt_utc
        .with_timezone(&Local)
        .format(FORMAT)
        .to_string()
        .serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    if let Ok(date) = NaiveDateTime::deserialize(deserializer) {
        let dt_local = date.and_local_timezone(Local).unwrap();
        Ok(dt_local.with_timezone(&Utc).naive_utc())
    } else {
        Err(serde::de::Error::custom("Invalid time format."))
    }
}

pub mod optional {
    use super::*;
    pub fn serialize<S>(opt: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = opt {
            let dt_utc = date.and_local_timezone(Utc).unwrap();
            dt_utc
                .with_timezone(&Local)
                .format(FORMAT)
                .to_string()
                .serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Ok(Some(date)) = Option::<NaiveDateTime>::deserialize(deserializer) {
            let dt_local = date.and_local_timezone(Local).unwrap();
            Ok(Some(dt_local.with_timezone(&Utc).naive_utc()))
        } else {
            Ok(None)
        }
    }
}
