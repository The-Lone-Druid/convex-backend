use std::{
    ops::Deref,
    str::FromStr,
};

use chrono::{
    DateTime,
    Utc,
};
use common::value::{
    ConvexObject,
    FieldPath,
    IdentifierFieldName,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::constants::{
    ID_FIELD_PATH,
    ID_FIVETRAN_FIELD_NAME,
    SOFT_DELETE_FIELD_PATH,
    SOFT_DELETE_FIVETRAN_FIELD_NAME,
    SYNCED_FIELD_PATH,
    SYNCED_FIVETRAN_FIELD_NAME,
};

#[derive(
    Hash, Eq, PartialEq, derive_more::Display, Debug, serde::Deserialize, Clone, PartialOrd, Ord,
)]
#[serde(transparent)]
pub struct FivetranTableName(String);

impl FromStr for FivetranTableName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl Deref for FivetranTableName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0[..]
    }
}

#[derive(
    Hash, Eq, PartialEq, derive_more::Display, Debug, serde::Deserialize, Clone, PartialOrd, Ord,
)]
#[serde(transparent)]
pub struct FivetranFieldName(String);

impl FromStr for FivetranFieldName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl Deref for FivetranFieldName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0[..]
    }
}

impl TryInto<FieldPath> for FivetranFieldName {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<FieldPath, Self::Error> {
        Ok(if &self == SYNCED_FIVETRAN_FIELD_NAME.deref() {
            SYNCED_FIELD_PATH.clone()
        } else if &self == SOFT_DELETE_FIVETRAN_FIELD_NAME.deref() {
            SOFT_DELETE_FIELD_PATH.clone()
        } else if &self == ID_FIVETRAN_FIELD_NAME.deref() {
            ID_FIELD_PATH.clone()
        } else {
            let field = IdentifierFieldName::from_str(&self)?;
            FieldPath::for_root_field(field)
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BatchWriteOperation {
    Upsert,
    Update,
    HardDelete,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchWriteRow {
    pub table: String,
    pub operation: BatchWriteOperation,
    pub row: ConvexObject,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum DeleteType {
    SoftDelete,
    HardDelete,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TruncateTableArgs {
    pub table_name: String,
    pub delete_before: Option<DateTime<Utc>>,
    pub delete_type: DeleteType,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use common::value::FieldPath;

    use crate::api_types::FivetranFieldName;

    #[test]
    fn convert_fivetran_user_fields_to_field_path() {
        let expected: FieldPath = FivetranFieldName::from_str("name")
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(expected, FieldPath::from_str("name").unwrap());
    }

    #[test]
    fn convert_fivetran_metadata_fields_to_field_path() {
        let expected: FieldPath = FivetranFieldName::from_str("_fivetran_synced")
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(expected, FieldPath::from_str("fivetran.synced").unwrap());

        let expected: FieldPath = FivetranFieldName::from_str("_fivetran_id")
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(expected, FieldPath::from_str("fivetran.id").unwrap());

        let expected: FieldPath = FivetranFieldName::from_str("_fivetran_deleted")
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(expected, FieldPath::from_str("fivetran.deleted").unwrap());
    }
}
