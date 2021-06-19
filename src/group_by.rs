use std::collections::HashMap;

pub trait GroupBy<T> {
    fn group_by<F>(self, fun: F) -> HashMap<String, Vec<T>>
    where
        Self: Sized,
        F: Fn(T) -> String;
}

impl<T: Sized + Clone> GroupBy<T> for &mut std::slice::Iter<'_, T> {
    fn group_by<F>(self, fun: F) -> HashMap<String, Vec<T>>
    where
        Self: Sized,
        F: Fn(T) -> String,
    {
        self.fold(HashMap::new(), |mut map, curr| {
            let entry = map.entry(fun(curr.clone())).or_insert_with(Vec::new);
            entry.push(curr.clone());
            map
        })
    }
}
