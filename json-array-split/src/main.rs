use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use serde_json::{Map, Value};

use traceback_error::{traceback, TracebackError};

use clap::Parser;

/// Splits a JSON array from a file into multiple smaller files.
///
/// The purpose of this function is to split a large JSON array stored in a file
/// into smaller files to make it manageable for upload or processing.
///
/// # Arguments
///
/// * `filepath` - A string representing the path to the input JSON file.
/// * `split_size` - The size of each split file.
///
/// # Returns
///
/// Returns `Ok(())` if the splitting process is successful, or `Err(TracebackError)` on failure.
///
/// # Possible Problems
///
/// This function can encounter the following issues:
///
/// - If the file cannot be read, the function will return an error.
/// - If the file is malformed JSON, the function will return an error.
/// - If the JSON file is not an array, the function will return an error.
/// - If writing to the split files fails, it will result in an error.
/// - If the file is too large and the host machine doesn't have enough memory,
///   it may lead to a panic.
/// - If the file is too large and the host machine doesn't have enough disk space,
///   it may also lead to a panic.
///
/// # Possible Improvements
///
/// Some possible improvements for this function include:
///
/// - Reducing code repetition to improve maintainability.
/// - General code cleanup and optimization.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use serde::Deserialize;
/// use traceback_error::TracebackError;
/// use your_module_name::split_array_from_json_file;
///
/// #[derive(Debug, Deserialize)]
/// struct Item {
///     // Define your struct fields here.
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), TracebackError> {
///     let file_path = "path/to/your/json_file.json";
///     let split_size = 100; // Specify the desired split size.
///
///     let result = split_array_from_json_file(file_path, split_size);
///
///     match result {
///         Ok(_) => {
///             println!("JSON array successfully split.");
///         }
///         Err(err) => {
///             eprintln!("Error: {:?}", err);
///         }
///     }
///
///     Ok(())
/// }
/// ```
///
/// In this example, the `split_array_from_json_file` function is used to split a JSON array from a file into smaller files.
/// Make sure to specify the correct file path and desired split size for your use case.
fn split_array_from_json_file(filepath: &str, split_size: usize) -> Result<(), TracebackError> {
    let str = match read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            return Err(traceback!(err e, "Error when reading roller JSON"));
        }
    };
    let parsed: serde_json::Value = match serde_json::from_str(&str) {
        Ok(p) => p,
        Err(e) => {
            return Err(traceback!(err e, "Error when parsing roller JSON"));
        }
    };
    let parsed = match parsed.as_array() {
        Some(p) => p,
        None => {
            return Err(traceback!("Error when parsing roller JSON: not an array"));
        }
    };
    let folder_path = filepath.split(".").collect::<Vec<&str>>()[filepath.split(".").count() - 2];
    let extension = filepath.split(".").collect::<Vec<&str>>()[filepath.split(".").count() - 1];
    let filename = filepath.split("/").collect::<Vec<&str>>()[filepath.split("/").count() - 1]
        .split(".")
        .collect::<Vec<&str>>()[0];
    println!("Path: .{folder_path}/{filename}/0.{extension}");
    println!("Folder path: {folder_path}");
    println!("Extension: {extension}");
    println!("Filename: {filename}");
    match create_dir_all(format!(".{folder_path}")) {
        Ok(_) => {}
        Err(e) => {
            return Err(traceback!(err e, "Error when creating directory"));
        }
    };
    let mut i = 0;
    let parsed_split = parsed.chunks(split_size);
    for chunk in parsed_split {
        let mut file = match File::create(format!(".{folder_path}/{i}.{extension}")) {
            Ok(f) => f,
            Err(e) => {
                return Err(traceback!(err e, "Error when creating file"));
            }
        };
        let chunk = match serde_json::to_string(chunk) {
            Ok(c) => c,
            Err(e) => {
                return Err(traceback!(err e, "Error when parsing chunk"));
            }
        };
        match file.write_all(chunk.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(traceback!(err e, "Error when writing to file"));
            }
        };
        i += 1;
    }
    Ok(())
}

#[macro_export]
macro_rules! extract_nested_json {
    ($json:expr, $ret_type:ty, $key:expr) => {
        || {
            let j = $json[$key].clone();
            let parsed_to_type: $ret_type = match serde_json::from_value(j) {
                Ok(v) => v,
                Err(e) => return Err(
                    traceback!(e,
                        format!(
                            "Error when getting key {key} from json {json} with expected type {type}",
                            key = $key,
                            json = $json,
                            type = stringify!($ret_type),
                        )
                    )
                ),
            };
            Ok(parsed_to_type)
        }
    };
    ($json:expr, $ret_type:ty, $key:expr, $($keys:expr),*) => {
        extract_nested_json!($json[$key], $ret_type, $($keys),*)
    };
}

/*
This function is used to detect nested json values.
It takes a json object, and returns the keys of all json values that are nested in the object.
It does this recursively, so if there are nested objects in the nested objects, it will return the keys of those as well.
This is returned as a vector of strings.
*/
fn detect_nested_json(json: &Map<String, Value>) -> Vec<String> {
    let mut keys = Vec::new();
    for (key, value) in json {
        match value {
            Value::Object(nested_obj) => {
                if !nested_obj.is_empty() {
                    let nested_keys = detect_nested_json(nested_obj);
                    for nested_key in nested_keys {
                        keys.push(format!("{}.{}", key, nested_key));
                    }
                }
            }
            _ => {
                keys.push(key.to_string());
            }
        }
    }
    keys
}

/// Generates a JSON schema from a given JSON-like data structure.
///
/// This function takes a reference to a JSON-like data structure represented by
/// the `serde_json::Value` enum and generates a JSON schema describing its structure.
/// The schema describes the types of values, arrays, and objects contained in the input
/// data, and their hierarchical relationships.
///
/// # Arguments
///
/// * `input`: A reference to the JSON-like data structure (`serde_json::Value`) for which
///   you want to generate a schema.
///
/// # Returns
///
/// A `serde_json::Value` representing the JSON schema.
///
/// # Example
///
/// ```rust
/// use serde_json::json;
///
/// let input_data = json!({
///     "name": "John",
///     "age": 30,
///     "is_student": false,
///     "hobbies": ["reading", "swimming"],
/// });
///
/// let schema = generate_schema(&input_data);
///
/// // The `schema` now contains a JSON schema describing the structure of `input_data`.
/// ```
///
/// # Note
///
/// This function recursively traverses the input data structure to generate the schema.
/// It supports various data types including null, boolean, number, string, arrays, and objects.
/// The generated schema is represented as a `serde_json::Value`.
fn generate_schema(input: &serde_json::Value) -> serde_json::Value {
    // Match the input value to determine its type and generate the schema accordingly
    match input {
        serde_json::Value::Null => serde_json::json!({"type": "null"}),
        serde_json::Value::Bool(_) => serde_json::json!({"type": "boolean"}),
        serde_json::Value::Number(_) => serde_json::json!({"type": "number"}),
        serde_json::Value::String(_) => serde_json::json!({"type": "string"}),
        serde_json::Value::Array(arr) => {
            // Generate the schema for array values
            let items_schema = arr.iter().fold(None, |schema, item| {
                let item_schema = generate_schema(item);
                match schema {
                    Some(schema) => {
                        if schema != item_schema {
                            Some(serde_json::json!([schema, item_schema]))
                        } else {
                            Some(schema)
                        }
                    }
                    None => Some(item_schema),
                }
            });

            serde_json::json!({
                "type": "array",
                "items": items_schema.unwrap_or(serde_json::json!({}))
            })
        }
        serde_json::Value::Object(obj) => {
            // Generate the schema for object values
            let properties: serde_json::Map<String, serde_json::Value> = obj
                .iter()
                .map(|(key, value)| {
                    let prop_schema = generate_schema(value);
                    (key.clone(), prop_schema)
                })
                .collect();

            serde_json::json!({
                "type": "object",
                "properties": properties,
                "required": obj.keys().cloned().collect::<Vec<String>>()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_generate_schema_null() {
        let input = Value::Null;
        let expected_schema = json!({"type": "null"});
        assert_eq!(generate_schema(&input), expected_schema);
    }

    #[test]
    fn test_generate_schema_boolean() {
        let input = Value::Bool(true);
        let expected_schema = json!({"type": "boolean"});
        assert_eq!(generate_schema(&input), expected_schema);
    }

    #[test]
    fn test_generate_schema_number() {
        let input = Value::Number(serde_json::Number::from(42));
        let expected_schema = json!({"type": "number"});
        assert_eq!(generate_schema(&input), expected_schema);
    }

    #[test]
    fn test_generate_schema_string() {
        let input = Value::String("Hello, World!".to_string());
        let expected_schema = json!({"type": "string"});
        assert_eq!(generate_schema(&input), expected_schema);
    }

    #[test]
    fn test_generate_schema_array() {
        let input = json!([1, 2, 3]);
        let expected_schema = json!({
            "type": "array",
            "items": {"type": "number"}
        });
        assert_eq!(generate_schema(&input), expected_schema);
    }

    #[test]
    fn test_generate_schema_object() {
        let input = json!({"key1": 42, "key2": "value"});
        let expected_schema = json!({
            "type": "object",
            "properties": {
                "key1": {"type": "number"},
                "key2": {"type": "string"}
            },
            "required": ["key1", "key2"]
        });
        assert_eq!(generate_schema(&input), expected_schema);
    }
}

/// Recursively compares two Serde JSON objects and identifies any differences.
///
/// This function takes two JSON objects represented as `serde_json::Value` and recursively compares
/// their structure and values. It returns an optional `serde_json::Map<String, Value>` containing the
/// differences found between the two objects. If no differences are found, it returns `None`.
///
/// # Arguments
///
/// * `a` - The first JSON object for comparison.
/// * `b` - The second JSON object for comparison.
///
/// # Returns
///
/// An optional `serde_json::Map<String, Value>` containing the differences found between `a` and `b`,
/// or `None` if no differences are found.
///
/// # Example
///
/// ```rust
/// use serde_json::{Value, Map};
/// use utils::json::compare_json_objects;
///
/// let json1: Value = serde_json::from_str(r#"
///     {
///         "name": "John",
///         "age": 25,
///         "address": {
///             "city": "Kandy",
///             "country": "Sri Lanka"
///         },
///         "species": "human"
///     }
/// "#).unwrap();
///
/// let json2: Value = serde_json::from_str(r#"
///     {
///         "name": "Jane",
///         "age": 30,
///         "address": {
///             "city": "Berlin",
///             "country": "Germany"
///         },
///         "species": "human"
///     }
/// "#).unwrap();
///
/// let expected_difference = serde_json::from_str(r#"
///     {
///         "name": {
///             "left": "John",
///             "right": "Jane"
///         },
///         "age": {
///             "left": 25,
///             "right": 30
///         },
///         "address": {
///             "city": {
///                 "left": "Kandy",
///                 "right": "Berlin"
///             },
///             "country": {
///                 "left": "Sri Lanka",
///                 "right": "Germany"
///             }
///         }
///     }
/// "#).unwrap();
///
/// assert_eq!(compare_json_objects(&json1, &json2), expected_difference)
/// ```
///
/// In the example above, two JSON objects are compared, and any differences between them are printed.
///
/// # Note
///
/// This function is intended for comparing small to moderately sized JSON objects. Performance may
/// degrade for very large or deeply nested JSON structures.
fn compare_json_objects(a: &Value, b: &Value) -> Option<Map<String, Value>> {
    match (a, b) {
        (Value::Object(obj1), Value::Object(obj2)) => {
            let mut diff = Map::new();

            for (key, value1) in obj1.iter() {
                if let Some(value2) = obj2.get(key) {
                    if let Some(sub_diff) = compare_json_objects(value1, value2) {
                        diff.insert(key.clone(), Value::Object(sub_diff));
                    }
                } else {
                    diff.insert(key.clone(), value1.clone());
                }
            }

            for (key, value2) in obj2.iter() {
                if !obj1.contains_key(key) {
                    diff.insert(key.clone(), value2.clone());
                }
            }

            if diff.is_empty() {
                None
            } else {
                Some(diff)
            }
        }
        _ => {
            if a != b {
                let mut diff = Map::new();
                diff.insert("left".to_string(), a.clone());
                diff.insert("right".to_string(), b.clone());
                Some(diff)
            } else {
                None
            }
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    in_file: PathBuf,

    #[arg(short, long)]
    out_folder: Option<PathBuf>,

    #[arg(long, conflicts_with = "split_size")]
    split_amount: Option<usize>,

    #[arg(long)]
    split_size: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let input: PathBuf = args.in_file;
    let output: PathBuf = match args.out_folder {
        Some(out) => out,
        None => derive_dest_from_src(&input).unwrap(),
    };

    println!(
        "In: {in_file:?}, out: {out_file:?}",
        in_file = input,
        out_file = output
    );

    println!("Creating folder {output:?}");
    create_dir_all(&output).unwrap();

    println!("Reading contents of {input:?}");
    let contents = std::fs::read_to_string(&input).unwrap();
    println!("Parsing to JSON...");
    let val: Value = serde_json::from_str(&contents).unwrap();
    println!("Parsing to array...");
    let parsed = val.as_array().unwrap();
    println!("Finished JSON parsing.");

    let chunk_size = match (args.split_amount, args.split_size) {
        (Some(_), Some(_)) => {
            panic!("It should be impossible to have both split-amount and split-size set")
        }
        (None, Some(num)) => num,
        (Some(num), None) => parsed.len() / num,
        (None, None) => 1,
    };

    println!("Splittng into chunks of size {chunk_size}");

    let parsed_split = parsed.chunks(chunk_size);
    let len = parsed_split.len() - 1;

    println!("Starting iteration through {len} chunks",);

    for (index, chunk) in parsed_split.enumerate() {
        println!("At index {index} of {len}");
        let mut output_clone = output.clone();
        output_clone.extend(vec![format!("{index}.json")]);
        println!("Making {output_clone:?}");
        let mut file = match File::create(&output_clone) {
            Ok(f) => f,
            Err(e) => {
                dbg!(e);
                return;
                //  Err(traceback!(err e, "Error when creating file"));
            }
        };
        println!("Made {output_clone:?}, stringifying JSON");
        let chunk = match serde_json::to_string(chunk) {
            Ok(c) => c,
            Err(e) => {
                dbg!(e);
                return;
                //  Err(traceback!(err e, "Error when parsing chunk"));
            }
        };
        println!("Stringifyed, writing to {output_clone:?}");
        match file.write_all(chunk.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                dbg!(e);
                return;
                //  Err(traceback!(err e, "Error when writing to file"));
            }
        };
        println!("Done with {index}");
    }
}

fn derive_dest_from_src(src: &PathBuf) -> Option<PathBuf> {
    let mut output = None;
    if let (Some(parent), Some(Some(mut filename))) = (
        src.parent(),
        src.file_name()
            .map(|ok| ok.to_str().map(|ok| ok.to_string())),
    ) {
        // remove extension if it exists
        if let Some(Some(ext)) = src.extension().map(|ok| ok.to_str()) {
            filename = filename
                .strip_suffix(&format!(".{ext}"))
                .unwrap()
                .to_string();
        }
        let mut parent = parent.to_path_buf();
        parent.extend(vec![filename]);
        println!("Path: {parent:?}");
        output = Some(parent);
    }

    output
}
