pub mod user;
pub mod ai;

mod datetime_format {
    use chrono::{DateTime, TimeZone, Utc};
    use mongodb::bson::DateTime as BsonDateTime;
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
    
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp_millis();
        let bson_dt = BsonDateTime::from_millis(timestamp);
        bson_dt.serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bson_dt = BsonDateTime::deserialize(deserializer)?;
        let millis = bson_dt.timestamp_millis();
        let dt = Utc.timestamp_millis_opt(millis).single()
            .ok_or_else(|| serde::de::Error::custom("无效的日期时间"))?;
        Ok(dt)
    }
}