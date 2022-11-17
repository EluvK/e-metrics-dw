pub trait SqlTable {
    type TypeSelf;
    fn new_sql_table_opt() -> &'static str;

    fn multi_insert_table_opt() -> &'static str;

    fn to_param_value_str(&self) -> String;
}
