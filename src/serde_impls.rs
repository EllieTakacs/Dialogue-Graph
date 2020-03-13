use crate::{
    condition::{And, Condition, Not, Or},
    DialogueGraph, Edge,
};
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;
use std::marker::PhantomData;

impl<'de, T> Deserialize<'de> for Edge<'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
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
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            phantom: PhantomData<&'de T>,
        };

        impl<'de, T> EdgeVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for EdgeVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
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
    T: Condition<'de> + Serialize + Deserialize<'de>,
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
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            phantom: PhantomData<&'de T>,
            phantom2: PhantomData<&'a T>,
        };

        impl<'a, 'de, T> DialogueGraphVisitor<'a, 'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
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
            T: Condition<'de> + Serialize + Deserialize<'de>,
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

impl<'de, T> Deserialize<'de> for Not<'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
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

        struct NotVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            phantom: PhantomData<&'de T>,
        };

        impl<'de, T> NotVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for NotVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            type Value = Not<'de, T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Not")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Not<'de, T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let condition = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Not::new(condition))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Not<'de, T>, V::Error>
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
                Ok(Not::new(condition))
            }
        }

        const FIELDS: &'static [&'static str] = &["condition"];
        deserializer.deserialize_struct("Not", FIELDS, NotVisitor::new())
    }
}

impl<'de, T> Deserialize<'de> for And<'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Left,
            Right,
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
                        formatter.write_str("`left` or `right`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "left" => Ok(Field::Left),
                            "right" => Ok(Field::Right),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct AndVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            phantom: PhantomData<&'de T>,
        };

        impl<'de, T> AndVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for AndVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            type Value = And<'de, T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct And")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<And<'de, T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let left = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let right = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(And::new(left, right))
            }

            fn visit_map<V>(self, mut map: V) -> Result<And<'de, T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut left: Option<T> = None;
                let mut right: Option<T> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Left => {
                            if left.is_some() {
                                return Err(de::Error::duplicate_field("left"));
                            }
                            left = Some(map.next_value()?);
                        }
                        Field::Right => {
                            if right.is_some() {
                                return Err(de::Error::duplicate_field("right"));
                            }
                            right = Some(map.next_value()?);
                        }
                    }
                }

                let left = left.ok_or_else(|| de::Error::missing_field("left"))?;
                let right = right.ok_or_else(|| de::Error::missing_field("right"))?;

                Ok(And::new(left, right))
            }
        }

        const FIELDS: &'static [&'static str] = &["left", "right"];
        deserializer.deserialize_struct("And", FIELDS, AndVisitor::new())
    }
}

impl<'de, T> Deserialize<'de> for Or<'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Left,
            Right,
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
                        formatter.write_str("`left` or `right`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "left" => Ok(Field::Left),
                            "right" => Ok(Field::Right),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct OrVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            phantom: PhantomData<&'de T>,
        };

        impl<'de, T> OrVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for OrVisitor<'de, T>
        where
            T: Condition<'de> + Serialize + Deserialize<'de>,
        {
            type Value = Or<'de, T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Or")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Or<'de, T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let left = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let right = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Or::new(left, right))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Or<'de, T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut left = None;
                let mut right = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Left => {
                            if left.is_some() {
                                return Err(de::Error::duplicate_field("left"));
                            }
                            left = Some(map.next_value()?);
                        }
                        Field::Right => {
                            if right.is_some() {
                                return Err(de::Error::duplicate_field("right"));
                            }
                            right = Some(map.next_value()?);
                        }
                    }
                }

                let left = left.ok_or_else(|| de::Error::missing_field("left"))?;
                let right = right.ok_or_else(|| de::Error::missing_field("right"))?;

                Ok(Or::new(left, right))
            }
        }

        const FIELDS: &'static [&'static str] = &["left", "right"];
        deserializer.deserialize_struct("Or", FIELDS, OrVisitor::new())
    }
}

// impl<'de, T, U> Deserialize<'de> for Function<'de, T, U>
// where
//     T: Serialize + Deserialize<'de>,
//     for<'a> U: FnOnce(&'a U) -> bool + Serialize + Deserialize<'de>,
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         enum Field {
//             Data,
//             Condition,
//         };
//
//         impl<'de> Deserialize<'de> for Field {
//             fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
//             where
//                 D: Deserializer<'de>,
//             {
//                 struct FieldVisitor;
//
//                 impl<'de> Visitor<'de> for FieldVisitor {
//                     type Value = Field;
//
//                     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                         formatter.write_str("`data` or `condition`")
//                     }
//
//                     fn visit_str<E>(self, value: &str) -> Result<Field, E>
//                     where
//                         E: de::Error,
//                     {
//                         match value {
//                             "data" => Ok(Field::Data),
//                             "condition" => Ok(Field::Condition),
//                             _ => Err(de::Error::unknown_field(value, FIELDS)),
//                         }
//                     }
//                 }
//
//                 deserializer.deserialize_identifier(FieldVisitor)
//             }
//         }
//
//         struct FunctionVisitor<'de, T, U>
//         where
//             T: Serialize + Deserialize<'de>,
//             for<'a> U: FnOnce(&'a U) -> bool + Serialize + Deserialize<'de>,
//         {
//             phantom: PhantomData<&'de T>,
//             phantom2: PhantomData<&'de U>,
//         };
//
//         impl<'de, T, U> FunctionVisitor<'de, T, U>
//         where
//             T: Serialize + Deserialize<'de>,
//             for<'a> U: FnOnce(&'a U) -> bool + Serialize + Deserialize<'de>,
//         {
//             fn new() -> Self {
//                 Self {
//                     phantom: PhantomData,
//                     phantom2: PhantomData,
//                 }
//             }
//         }
//
//         impl<'de, T, U> Visitor<'de> for FunctionVisitor<'de, T, U>
//         where
//             T: Serialize + Deserialize<'de>,
//             U: FnOnce(&U) -> bool + Serialize + Deserialize<'de>,
//         {
//             type Value = Function<'de, T, U>;
//
//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("struct Function")
//             }
//
//             fn visit_seq<V>(self, mut seq: V) -> Result<Function<'de, T, U>, V::Error>
//             where
//                 V: SeqAccess<'de>,
//             {
//                 let data = seq
//                     .next_element()?
//                     .ok_or_else(|| de::Error::invalid_length(0, &self))?;
//                 let condition = seq
//                     .next_element()?
//                     .ok_or_else(|| de::Error::invalid_length(1, &self))?;
//                 Ok(Function {
//                     data,
//                     condition,
//                     phantom: PhantomData,
//                     phantom2: PhantomData,
//                 })
//             }
//
//             fn visit_map<V>(self, mut map: V) -> Result<Function<'de, T, U>, V::Error>
//             where
//                 V: MapAccess<'de>,
//             {
//                 let mut data = None;
//                 let mut condition = None;
//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         Field::Data => {
//                             if data.is_some() {
//                                 return Err(de::Error::duplicate_field("data"));
//                             }
//                             data = Some(map.next_value()?);
//                         }
//                         Field::Condition => {
//                             if condition.is_some() {
//                                 return Err(de::Error::duplicate_field("condition"));
//                             }
//                         }
//                     }
//                 }
//
//                 let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
//                 let condition = condition.ok_or_else(|| de::Error::missing_field("condition"))?;
//
//                 Ok(Function {
//                     data,
//                     condition,
//                     phantom: PhantomData,
//                     phantom2: PhantomData,
//                 })
//             }
//         }
//
//         const FIELDS: &'static [&'static str] = &["data", "condition"];
//         deserializer.deserialize_struct("Function", FIELDS, FunctionVisitor::new())
//     }
// }
