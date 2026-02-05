//! Build script for myhospitals-api
//!
//! This script preprocesses the OpenAPI specification to fix various issues
//! before generating the Rust client with Progenitor.
//!
//! ## Debugging
//!
//! Set `DEBUG_CODEGEN=1` to enable verbose output and write the modified spec:
//! ```sh
//! DEBUG_CODEGEN=1 cargo build
//! ```

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let spec_path = "spec/swagger.json";
    let debug = env::var("DEBUG_CODEGEN").is_ok();

    println!("cargo:rerun-if-changed={}", spec_path);

    // Read the OpenAPI spec
    let spec_content = fs::read_to_string(spec_path).unwrap_or_else(|e| {
        panic!(
            "Failed to read OpenAPI spec at '{}': {}\n\
             Hint: Make sure the spec/swagger.json file exists.",
            spec_path, e
        )
    });

    let mut spec_json: serde_json::Value = serde_json::from_str(&spec_content).unwrap_or_else(|e| {
        panic!(
            "Failed to parse OpenAPI spec as JSON: {}\n\
             Hint: The spec file may be corrupted or invalid.",
            e
        )
    });

    // Fix date-time formats to date for known date fields
    fix_date_formats(&mut spec_json);

    // Add VersionInformation schema to components
    // NOTE: This schema is not in the upstream OpenAPI spec but the API returns it
    // in all responses. If the API changes this structure, update here.
    // Last verified: 2026-02-05
    add_version_information_schema(&mut spec_json);

    // Process paths to add operationId and normalize content types
    process_paths(&mut spec_json);

    // Get output directory
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| {
        panic!("OUT_DIR environment variable not set. This script must be run by cargo.")
    });

    // Optionally write modified spec for debugging
    if debug {
        let debug_spec_path = Path::new(&out_dir).join("modified_spec.json");
        fs::write(
            &debug_spec_path,
            serde_json::to_string_pretty(&spec_json).unwrap(),
        )
        .unwrap_or_else(|e| {
            eprintln!("Warning: Failed to write debug spec: {}", e);
        });
        println!("cargo:warning=Debug spec written to {:?}", debug_spec_path);
    }

    // Parse as OpenAPI
    let spec: openapiv3::OpenAPI = serde_json::from_value(spec_json).unwrap_or_else(|e| {
        panic!(
            "Failed to parse modified spec as OpenAPI: {}\n\
             Hint: The spec transformations may have produced invalid OpenAPI.",
            e
        )
    });

    // Configure Progenitor
    let mut settings = progenitor::GenerationSettings::default();
    settings.with_interface(progenitor::InterfaceStyle::Builder);

    let mut generator = progenitor::Generator::new(&settings);

    // Generate code
    let tokens = generator.generate_tokens(&spec).unwrap_or_else(|e| {
        panic!(
            "Failed to generate API client code: {}\n\
             Hint: Check the OpenAPI spec for unsupported features.",
            e
        )
    });

    let ast = syn::parse2(tokens).unwrap_or_else(|e| {
        panic!(
            "Failed to parse generated code as Rust: {}\n\
             Hint: This is likely a Progenitor bug.",
            e
        )
    });

    let formatted_code = prettyplease::unparse(&ast);

    // Write generated code
    let out_path = Path::new(&out_dir).join("codegen.rs");
    fs::write(&out_path, formatted_code).unwrap_or_else(|e| {
        panic!("Failed to write generated code to {:?}: {}", out_path, e)
    });

    if debug {
        println!("cargo:warning=Generated API client at {:?}", out_path);
    }
}

/// Add VersionInformation schema that the API returns but isn't in the spec.
///
/// The MyHospitals API wraps all responses in an envelope:
/// ```json
/// {
///   "result": <actual data>,
///   "version_information": {
///     "api_version": "1.0",
///     "data_version": 123,
///     "date_uploaded": "...",
///     "requested_time_stamp": "..."
///   }
/// }
/// ```
///
/// This schema is not documented in the OpenAPI spec, so we add it manually.
fn add_version_information_schema(spec_json: &mut serde_json::Value) {
    if let Some(components) = spec_json.get_mut("components").and_then(|c| c.as_object_mut()) {
        if let Some(schemas) = components.get_mut("schemas").and_then(|s| s.as_object_mut()) {
            if !schemas.contains_key("VersionInformation") {
                schemas.insert(
                    "VersionInformation".to_string(),
                    serde_json::json!({
                        "type": "object",
                        "properties": {
                            "api_version": { "type": "string" },
                            "data_version": { "type": "integer" },
                            // Use plain strings - the API returns non-standard datetime formats
                            "date_uploaded": { "type": "string" },
                            "requested_time_stamp": { "type": "string" }
                        }
                    }),
                );
            }
        }
    }
}

/// Process all paths to add operationId and normalize content types.
fn process_paths(spec_json: &mut serde_json::Value) {
    let Some(paths) = spec_json.get_mut("paths").and_then(|p| p.as_object_mut()) else {
        return;
    };

    for (path, path_item) in paths.iter_mut() {
        let Some(path_obj) = path_item.as_object_mut() else {
            continue;
        };

        for method in ["get", "post", "put", "delete", "patch"] {
            let Some(operation) = path_obj.get_mut(method).and_then(|o| o.as_object_mut()) else {
                continue;
            };

            // Add operationId if missing (required by Progenitor)
            if !operation.contains_key("operationId") {
                let operation_id = generate_operation_id(path, method);
                operation.insert(
                    "operationId".to_string(),
                    serde_json::Value::String(operation_id),
                );
            }

            // Process responses
            let Some(responses) = operation.get_mut("responses").and_then(|r| r.as_object_mut())
            else {
                continue;
            };

            // Remove error response content to avoid multiple response type issues
            let error_statuses: Vec<String> = responses
                .keys()
                .filter(|k| k.starts_with('4') || k.starts_with('5'))
                .cloned()
                .collect();

            for status in error_statuses {
                if let Some(resp) = responses.get_mut(&status) {
                    if let Some(resp_obj) = resp.as_object_mut() {
                        resp_obj.remove("content");
                    }
                }
            }

            // Process success responses
            for (status, response) in responses.iter_mut() {
                if status.starts_with('4') || status.starts_with('5') {
                    continue;
                }

                let Some(content) = response.get_mut("content").and_then(|c| c.as_object_mut())
                else {
                    continue;
                };

                // Check if this is an Excel download endpoint
                let has_excel = content
                    .contains_key("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet");

                if has_excel {
                    // Convert Excel content type to application/octet-stream
                    content.clear();
                    content.insert(
                        "application/octet-stream".to_string(),
                        serde_json::json!({
                            "schema": {
                                "type": "string",
                                "format": "binary"
                            }
                        }),
                    );
                } else {
                    // Keep only application/json, remove text/csv and others
                    let keys_to_remove: Vec<String> = content
                        .keys()
                        .filter(|k| *k != "application/json")
                        .cloned()
                        .collect();

                    for key in keys_to_remove {
                        content.remove(&key);
                    }

                    // Wrap JSON schema in API envelope format
                    if let Some(json_content) = content.get_mut("application/json") {
                        if let Some(schema) = json_content.get("schema").cloned() {
                            let wrapped_schema = serde_json::json!({
                                "type": "object",
                                "properties": {
                                    "result": schema,
                                    "version_information": {
                                        "$ref": "#/components/schemas/VersionInformation"
                                    }
                                },
                                "required": ["result"]
                            });
                            if let Some(obj) = json_content.as_object_mut() {
                                obj.insert("schema".to_string(), wrapped_schema);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Fix date-time formats to date for fields that actually return dates.
///
/// The OpenAPI spec incorrectly marks several date fields as `format: date-time`
/// when the API actually returns date strings like "2012-06-30".
///
/// Known date fields (by description pattern):
/// - reporting_start_date, reporting_end_date
/// - Any field with "date" in description but not "time" or "stamp"
///
/// To update this list:
/// 1. Check API responses for fields returning "YYYY-MM-DD" format
/// 2. Update the heuristic below or add explicit field names
fn fix_date_formats(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            // Check if this is a property with format: date-time
            if let Some(format) = map.get("format") {
                if format == "date-time" {
                    // Check if this is a date field (not a timestamp)
                    if let Some(description) = map.get("description") {
                        let desc = description.as_str().unwrap_or("");
                        // Date fields have "date" in description but not "time" or "stamp"
                        // This matches: reporting_start_date, reporting_end_date, etc.
                        if (desc.to_lowercase().contains("date"))
                            && !desc.to_lowercase().contains("time")
                            && !desc.to_lowercase().contains("stamp")
                        {
                            map.insert(
                                "format".to_string(),
                                serde_json::Value::String("date".to_string()),
                            );
                        }
                    }
                }
            }
            // Recurse into all values
            for v in map.values_mut() {
                fix_date_formats(v);
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr.iter_mut() {
                fix_date_formats(item);
            }
        }
        _ => {}
    }
}

/// Generate operationId from path and method.
///
/// Converts "/api/v1/caveats/{caveat-code}" + "get" to "get_caveats_by_caveat_code"
fn generate_operation_id(path: &str, method: &str) -> String {
    let path_parts: Vec<&str> = path
        .trim_start_matches("/api/v1/")
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let mut parts = vec![method.to_string()];

    for part in path_parts {
        if part.starts_with('{') && part.ends_with('}') {
            // Path parameter like {caveat-code}
            let param = part
                .trim_start_matches('{')
                .trim_end_matches('}')
                .replace('-', "_");
            parts.push(format!("by_{}", param));
        } else {
            parts.push(part.replace('-', "_"));
        }
    }

    parts.join("_")
}
