use sea_orm_migration::{prelude::*, schema::*};

pub const CUID2_LENGTH: u32 = 24;

pub fn pg_primary_id<T: IntoIden>(col: T) -> ColumnDef {
    string_len(col, CUID2_LENGTH)
        .primary_key()
        .not_null()
        .to_owned()
}

pub fn pg_id<T: IntoIden>(col: T) -> ColumnDef {
    string_len(col, CUID2_LENGTH).not_null().to_owned()
}

pub fn pg_id_null<T: IntoIden>(col: T) -> ColumnDef {
    string_len_null(col, CUID2_LENGTH).to_owned()
}

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

// pub fn jsonb_arr_default<T: IntoIden>(col: T) -> ColumnDef {
//     json_binary(col).default("[]").to_owned()
// }
