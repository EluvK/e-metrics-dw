pub trait SqlTable {
    type TypeSelf;
    fn new_sql_table_opt() -> &'static str;

    fn insert_table_opt() -> &'static str;

    fn to_params(&self) -> mysql_async::Params;
}
