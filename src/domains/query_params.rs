use crate::domains::DateConstraintString;
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::fmt;

#[derive(Debug)]
pub struct QueryParams {
    pub since: Option<DateConstraintString>,
    pub upto: Option<DateConstraintString>,
}

impl QueryParams {
    pub fn new(since: Option<DateConstraintString>, upto: Option<DateConstraintString>) -> Self {
        QueryParams { since, upto }
    }
}

impl<'de> Deserialize<'de> for QueryParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Since,
            Upto,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`since` or `upto`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "since" => Ok(Field::Since),
                            "upto" => Ok(Field::Upto),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct QueryParamsVisitor;

        impl<'de> Visitor<'de> for QueryParamsVisitor {
            type Value = QueryParams;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct QueryParams")
            }

            fn visit_map<V>(self, mut map: V) -> Result<QueryParams, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut since = None;
                let mut upto = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Since => {
                            if since.is_some() {
                                return Err(de::Error::duplicate_field("since"));
                            }

                            let value = map.next_value::<String>()?;
                            let value = DateConstraintString::parse(value).map_err(|val| {
                                de::Error::invalid_value(
                                    de::Unexpected::Other(&val),
                                    &"valid query param string",
                                )
                            })?;

                            since = Some(value);
                        }
                        Field::Upto => {
                            if upto.is_some() {
                                return Err(de::Error::duplicate_field("upto"));
                            }

                            let value = map.next_value::<String>()?;
                            let value = DateConstraintString::parse(value).map_err(|val| {
                                de::Error::invalid_value(
                                    de::Unexpected::Other(&val),
                                    &"valid query param string",
                                )
                            })?;

                            upto = Some(value);
                        }
                    }
                }

                Ok(QueryParams::new(since, upto))
            }
        }

        const FIELDS: &'static [&'static str] = &["since", "upto"];
        deserializer.deserialize_struct("QueryParams", FIELDS, QueryParamsVisitor)
    }
}
