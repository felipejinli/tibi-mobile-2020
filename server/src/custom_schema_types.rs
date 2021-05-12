use diesel::backend::Backend;
use diesel::pg::Pg;
use diesel::sql_types::{Record, Varchar};
use diesel::{
    deserialize::FromSql,
    serialize::{self, ToSql},
};
use std::io::Write;

pub mod export {
    pub use super::Lineupitem;
    pub use diesel::sql_types::*;
}

#[derive(Debug, QueryId, AsExpression, FromSqlRow)]
pub struct LineupItem(pub String, pub String);
// {
//     pub label: String,
//     pub description: String,
// }

impl FromSql<LineupItemType, Pg> for LineupItem {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> diesel::deserialize::Result<Self> {
        let (l, d) = FromSql::<Record<(Varchar, Varchar)>, Pg>::from_sql(bytes)?;

        Ok(LineupItem(l, d))
    }
}

// impl ToSql<diesel::sql_types::Uuid, Pg> for PostId {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> diesel::serialize::Result {
//         <uuid::Uuid as ToSql<diesel::sql_types::Uuid, Pg>>::to_sql(&self.0, out)
//     }
// }

impl ToSql<LineupItemType, Pg> for LineupItem {
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Pg>) -> serialize::Result {
        serialize::WriteTuple::<(Varchar, Varchar)>::write_tuple(
            &(self.0.clone(), self.1.clone()),
            out,
        )
    }
}

#[derive(SqlType, QueryId)]
#[postgres(type_name = "Lineupitem")]
pub struct LineupItemType;

// Diesel setup forcings capitalisation of first letter then lowercase of the rest:
pub type Lineupitem = LineupItemType;
