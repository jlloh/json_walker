use serde_json::Value;

/// Get leaf node keys from a json string, in the form of an array of vec!["a.b", "a.c.d"]
pub fn get_leafs_from_json(input: &str, root: &str) -> Vec<(String, bool)> {
    let v: Value = serde_json::from_str(input).unwrap();
    let mut vec = Vec::new();
    walk(v, root, &mut vec);
    vec
}

/// Recursively walks through a Json Value object to arrive at the leaf nodes, and to append to a list
/// TODO: Construct a graph from petgraphs also?
fn walk(input: Value, root: &str, array: &mut Vec<(String, bool)>) {
    match input {
        // Value::Null => println!("{}", root),
        // Recurse
        Value::Object(inside) => {
            for (k, v) in inside {
                let root_to_pass = if root.is_empty() {
                    // if root is empty, don't prefix with .
                    k
                } else {
                    format!("{}.{}", root, k)
                };
                walk(v, &root_to_pass, array)
            }
        }
        // Special treatment for arrays since they have multiple elements inside
        Value::Array(_) => array.push((root.to_string(), true)),
        // Reach the leaf of the json tree
        _ => array.push((root.to_string(), false)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn it_works_with_root() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "nested": {
              "inside_key": "inside_value",
              "inside_key_2": "inside_value_2",
              "nested_nested": {"inner_key": "inner_inside_value_2"}
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let results = get_leafs_from_json(data, "d");

        let expected = HashSet::<(String, bool)>::from_iter(
            vec![
                ("d.name", false),
                ("d.age", false),
                ("d.nested.inside_key", false),
                ("d.nested.inside_key_2", false),
                ("d.nested.nested_nested.inner_key", false),
                // This is an array and should be flagged as true
                ("d.phones", true),
            ]
            .into_iter()
            .map(|(x, y)| (x.to_string(), y)),
        );
        println!("{:?}", results);
        assert_eq!(HashSet::from_iter(results), expected)
    }

    #[test]
    fn it_works_with_without_root() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "nested": {
              "inside_key": "inside_value",
              "inside_key_2": "inside_value_2",
              "nested_nested": {"inner_key": "inner_inside_value_2"}
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let results = get_leafs_from_json(data, "");

        let expected = HashSet::<(String, bool)>::from_iter(
            vec![
                ("name", false),
                ("age", false),
                ("nested.inside_key", false),
                ("nested.inside_key_2", false),
                ("nested.nested_nested.inner_key", false),
                // This is an array and should be flagged as true
                ("phones", true),
            ]
            .into_iter()
            .map(|(x, y)| (x.to_string(), y)),
        );
        println!("{:?}", results);
        assert_eq!(HashSet::from_iter(results), expected)
    }
}
