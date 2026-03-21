use sea_orm_migration::{prelude::*, schema::*};

pub fn timestampz<T: IntoIden>(col: T) -> ColumnDef {
    timestamp_with_time_zone(col).to_owned()
}

pub fn timestampz_null<T: IntoIden>(col: T) -> ColumnDef {
    timestamp_with_time_zone_null(col).to_owned()
}

pub fn timestampz_default<T: IntoIden>(col: T) -> ColumnDef {
    timestamp_with_time_zone(col)
        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
        .to_owned()
}

pub fn jsonb_default<T: IntoIden>(col: T) -> ColumnDef {
    json_binary(col).default("{}").to_owned()
}

pub fn db_money<T: IntoIden>(col: T) -> ColumnDef {
    decimal_len(col, 78, 0)
}

pub fn db_money_null<T: IntoIden>(col: T) -> ColumnDef {
    decimal_len_null(col, 78, 0)
}

// pub fn jsonb_arr_default<T: IntoIden>(col: T) -> ColumnDef {
//     json_binary(col).default("[]").to_owned()
// }
