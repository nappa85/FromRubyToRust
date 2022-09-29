use std::ops::{Deref, DerefMut};

use sea_orm::{
    sea_query::{ColumnType, Nullable, ValueType, ValueTypeErr},
    DbErr, QueryResult, TryGetable, Value,
};

use serde::{de::DeserializeOwned, Serialize};

use tracing::error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Default> Default for Json<T> {
    fn default() -> Self {
        Json(Default::default())
    }
}

impl<'a, T: DeserializeOwned> TryFrom<&'a str> for Json<T> {
    type Error = serde_path_to_error::Error<serde_json::Error>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut deserializer = serde_json::Deserializer::from_str(value);
        Ok(Json(serde_path_to_error::deserialize(&mut deserializer)?))
    }
}

impl<T: Serialize> From<Json<T>> for Value {
    fn from(b: Json<T>) -> Self {
        Value::from(serde_json::to_string(&*b).unwrap())
    }
}

impl<T> Nullable for Json<T> {
    fn null() -> Value {
        Value::String(None)
    }
}

impl<T: DeserializeOwned> ValueType for Json<T> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(s)) => s.as_str().try_into().map_err(|e| {
                error!("{}", e);
                ValueTypeErr
            }),
            _ => {
                error!("type error");
                Err(ValueTypeErr)
            }
        }
    }
    fn column_type() -> ColumnType {
        ColumnType::String(Some(255))
    }
    fn type_name() -> String {
        "Json".to_owned()
    }
}

impl<T: DeserializeOwned> TryGetable for Json<T> {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, sea_orm::TryGetError> {
        let i = String::try_get(res, pre, col)?;
        i.as_str()
            .try_into()
            .map_err(|e| sea_orm::TryGetError::DbErr(DbErr::Query(format!("{}", e))))
    }
}
