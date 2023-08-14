#[macro_export]
macro_rules! data_map {
    () => {};

    ($( $key: expr => $val: expr ),*) => {{
        let mut map = DataMap::new();
        $( map.insert($key.into(), $val.into()); )*
        map
    }};
}
