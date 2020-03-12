use crate::{condition::Condition, DialogueGraph, Edge};
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;
use std::marker::PhantomData;

impl<'de, T> Deserialize<'de> for Edge<'de, T>
where
    T: Condition + Serialize + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Condition,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`condition`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "condition" => Ok(Field::Condition),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct EdgeVisitor<'de, T>
        where
            T: Condition + Serialize + Deserialize<'de>,
        {
            phantom: PhantomData<&'de T>,
        };

        impl<'de, T> EdgeVisitor<'de, T>
        where
            T: Condition + Serialize + Deserialize<'de>,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for EdgeVisitor<'de, T>
        where
            T: Condition + Serialize + Deserialize<'de>,
        {
            type Value = Edge<'de, T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Edge")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Edge<'de, T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let condition = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Edge {
                    condition,
                    phantom: PhantomData,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Edge<'de, T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut condition = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Condition => {
                            if condition.is_some() {
                                return Err(de::Error::duplicate_field("condition"));
                            }
                            condition = Some(map.next_value()?);
                        }
                    }
                }

                let condition = condition.ok_or_else(|| de::Error::missing_field("condition"))?;
                Ok(Edge {
                    condition,
                    phantom: PhantomData,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["condition"];
        deserializer.deserialize_struct("Edge", FIELDS, EdgeVisitor::new())
    }
}

impl<'a, 'de, T> Deserialize<'de> for DialogueGraph<'a, 'de, T>
where
    T: Condition + Serialize + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Data,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`condition`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "data" => Ok(Field::Data),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DialogueGraphVisitor<'a, 'de, T>
        where
            T: Condition + Serialize + Deserialize<'de>,
        {
            phantom: PhantomData<&'de T>,
            phantom2: PhantomData<&'a T>,
        };

        impl<'a, 'de, T> DialogueGraphVisitor<'a, 'de, T>
        where
            T: Condition + Serialize + Deserialize<'de>,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                    phantom2: PhantomData,
                }
            }
        }

        impl<'a, 'de, T> Visitor<'de> for DialogueGraphVisitor<'a, 'de, T>
        where
            T: Condition + Serialize + Deserialize<'de>,
        {
            type Value = DialogueGraph<'a, 'de, T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Edge")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<DialogueGraph<'a, 'de, T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let data = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(DialogueGraph {
                    data,
                    phantom: PhantomData,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<DialogueGraph<'a, 'de, T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut data = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("condition"));
                            }
                            data = Some(map.next_value()?);
                        }
                    }
                }

                let data = data.ok_or_else(|| de::Error::missing_field("condition"))?;
                Ok(DialogueGraph {
                    data,
                    phantom: PhantomData,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["data"];
        deserializer.deserialize_struct("DialogueGraph", FIELDS, DialogueGraphVisitor::new())
    }
}
