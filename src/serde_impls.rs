use crate::{
    condition::{And, Condition, Function, Not, Or},
    DialogueGraph, Edge,
};
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;
use std::marker::PhantomData;

impl<'de, T> Deserialize<'de> for Edge<T>
where
    T: Condition,
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

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        struct EdgeVisitor<T>
        where
            T: Condition,
        {
            phantom: PhantomData<T>,
        };

        impl<T> EdgeVisitor<T>
        where
            T: Condition,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for EdgeVisitor<T>
        where
            T: Condition,
        {
            type Value = Edge<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Edge")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Edge<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let condition = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Edge::new(condition))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Edge<T>, V::Error>
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
                Ok(Edge::new(condition))
            }
        }

        const FIELDS: &[&str] = &["condition"];
        deserializer.deserialize_struct("Edge", FIELDS, EdgeVisitor::new())
    }
}

impl<'de, T> Deserialize<'de> for DialogueGraph<T>
where
    T: Condition,
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

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        struct DialogueGraphVisitor<T>
        where
            T: Condition,
        {
            phantom: PhantomData<T>,
        };

        impl<T> DialogueGraphVisitor<T>
        where
            T: Condition,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for DialogueGraphVisitor<T>
        where
            T: Condition,
        {
            type Value = DialogueGraph<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Edge")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<DialogueGraph<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let data = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(DialogueGraph { data })
            }

            fn visit_map<V>(self, mut map: V) -> Result<DialogueGraph<T>, V::Error>
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
                Ok(DialogueGraph { data })
            }
        }

        const FIELDS: &[&str] = &["data"];
        deserializer.deserialize_struct("DialogueGraph", FIELDS, DialogueGraphVisitor::new())
    }
}

impl<'de, T> Deserialize<'de> for Not<T>
where
    T: Condition,
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

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        struct NotVisitor<T>
        where
            T: Condition,
        {
            phantom: PhantomData<T>,
        };

        impl<T> NotVisitor<T>
        where
            T: Condition,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for NotVisitor<T>
        where
            T: Condition,
        {
            type Value = Not<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Not")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Not<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let condition = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Not::new(condition))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Not<T>, V::Error>
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

        const FIELDS: &[&str] = &["condition"];
        deserializer.deserialize_struct("Not", FIELDS, NotVisitor::new())
    }
}

impl<'de, T> Deserialize<'de> for And<T>
where
    T: Condition,
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

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        struct AndVisitor<T>
        where
            T: Condition,
        {
            phantom: PhantomData<T>,
        };

        impl<T> AndVisitor<T>
        where
            T: Condition,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for AndVisitor<T>
        where
            T: Condition,
        {
            type Value = And<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct And")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<And<T>, V::Error>
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

            fn visit_map<V>(self, mut map: V) -> Result<And<T>, V::Error>
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

        const FIELDS: &[&str] = &["left", "right"];
        deserializer.deserialize_struct("And", FIELDS, AndVisitor::new())
    }
}

impl<'de, T> Deserialize<'de> for Or<T>
where
    T: Condition,
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

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        struct OrVisitor<T>
        where
            T: Condition,
        {
            phantom: PhantomData<T>,
        };

        impl<T> OrVisitor<T>
        where
            T: Condition,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for OrVisitor<T>
        where
            T: Condition,
        {
            type Value = Or<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Or")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Or<T>, V::Error>
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

            fn visit_map<V>(self, mut map: V) -> Result<Or<T>, V::Error>
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

        const FIELDS: &[&str] = &["left", "right"];
        deserializer.deserialize_struct("Or", FIELDS, OrVisitor::new())
    }
}

#[allow(single_use_lifetimes)]
impl<'de, T, U> Deserialize<'de> for Function<T, U>
where
    T: Serialize + for<'a> Deserialize<'a>,
    U: Fn(&T) -> bool + Serialize + for<'a> Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Data,
            Condition,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("`data` or `condition`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "data" => Ok(Field::Data),
                            "condition" => Ok(Field::Condition),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct FunctionVisitor<T, U>
        where
            T: Serialize + for<'de> Deserialize<'de>,
            U: Fn(&T) -> bool + Serialize + for<'de> Deserialize<'de>,
        {
            phantom: PhantomData<T>,
            phantom2: PhantomData<U>,
        };

        impl<T, U> FunctionVisitor<T, U>
        where
            T: Serialize + for<'de> Deserialize<'de>,
            U: Fn(&T) -> bool + Serialize + for<'de> Deserialize<'de>,
        {
            fn new() -> Self {
                Self {
                    phantom: PhantomData,
                    phantom2: PhantomData,
                }
            }
        }

        impl<'de, T, U> Visitor<'de> for FunctionVisitor<T, U>
        where
            T: Serialize + for<'a> Deserialize<'a>,
            U: Fn(&T) -> bool + Serialize + for<'a> Deserialize<'a>,
        {
            type Value = Function<T, U>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Function")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Function<T, U>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let data = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let condition = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Function::new(data, condition))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Function<T, U>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut data = None;
                let mut condition = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                        Field::Condition => {
                            if condition.is_some() {
                                return Err(de::Error::duplicate_field("condition"));
                            }
                            condition = Some(map.next_value()?);
                        }
                    }
                }

                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let condition = condition.ok_or_else(|| de::Error::missing_field("condition"))?;

                Ok(Function::new(data, condition))
            }
        }

        const FIELDS: &[&str] = &["data", "condition"];
        deserializer.deserialize_struct("Function", FIELDS, FunctionVisitor::new())
    }
}
