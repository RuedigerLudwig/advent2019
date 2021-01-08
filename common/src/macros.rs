// https://stackoverflow.com/a/28392068
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}

#[macro_export]
macro_rules! hashset {
    ($( $key: expr ),*) => {{
        let mut set = ::std::collections::HashSet::new();
        $( set.insert($key); )*
        set
    }}
}
