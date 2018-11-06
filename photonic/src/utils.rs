pub fn combine_opts<V, F>(v1: Option<V>, v2: Option<V>, f: F) -> Option<V>
    where F: FnOnce(V, V) -> V {
    match (v1, v2) {
        (Some(v1), Some(v2)) => Some(f(v1, v2)),
        (Some(v1), None) => Some(v1),
        (None, Some(v2)) => Some(v2),
        (None, None) => None,
    }
}

