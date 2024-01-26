#![allow(dead_code)]

use std::collections::HashMap;

use crate::Result;
use serde::{Deserialize, Serialize};

pub(crate) fn parse_schema(schema: &[u8]) -> Result<Vec<SchemaField>> {
    let json_schema: JsonSchema = serde_json::from_slice(schema)?;
    Ok(convert_schema_to_fields("", &json_schema))
}

#[derive(Serialize, Debug, Default)]
pub struct SchemaField {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub required: bool,
    pub deprecated: bool,
    pub children: String,
}

#[derive(Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    String,
    Number,
    Integer,
    Boolean,
    #[default]
    Object,
    Array,
    Null,
}

impl ToString for SchemaType {
    fn to_string(&self) -> String {
        match self {
            SchemaType::String => "String".to_string(),
            SchemaType::Number => "Number".to_string(),
            SchemaType::Integer => "Integer".to_string(),
            SchemaType::Boolean => "Boolean".to_string(),
            SchemaType::Object => "Object".to_string(),
            SchemaType::Array => "Array".to_string(),
            SchemaType::Null => "null".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub(crate) struct JsonSchema {
    pub description: String,
    #[serde(rename = "type")]
    pub data_type: SchemaType,
    pub properties: HashMap<String, JsonSchema>,
    pub items: Option<Box<JsonSchema>>,
    pub required: Vec<String>,
    pub format: String,
}

fn convert_schema_to_fields(prefix: &str, schema: &JsonSchema) -> Vec<SchemaField> {
    let mut fields = Vec::new();
    for (name, property) in &schema.properties {
        fields.append(&mut parse_property(
            &schema.required,
            prefix,
            (name, property),
        ));
    }
    fields.sort_by_key(|f| f.name.clone());
    fields
}

fn parse_property(
    required: &[String],
    prefix: &str,
    (name, schema): (&String, &JsonSchema),
) -> Vec<SchemaField> {
    let mut fields: Vec<SchemaField> = vec![];

    let required = required.contains(name);

    let name = if prefix.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", prefix, name)
    };
    let mut root_field = SchemaField {
        name: name.clone(),
        children: schema.description.to_string(),
        data_type: parse_type(schema),
        required,
        deprecated: false,
    };

    if schema.format.is_empty() {
        root_field
            .children
            .push_str(format!("\n\n---\n**Format:** {}\n", schema.format).as_str());
    }

    fields.push(root_field);

    if schema.data_type == SchemaType::Object {
        fields.append(&mut convert_schema_to_fields(&name, schema));
    }

    if schema.data_type == SchemaType::Array {
        if let Some(ref items) = schema.items {
            if items.data_type == SchemaType::Object {
                fields.append(&mut convert_schema_to_fields(
                    &name,
                    schema.items.as_ref().unwrap(),
                ));
            }
        }
    }

    fields
}

fn parse_type(schema: &JsonSchema) -> String {
    match schema.data_type {
        SchemaType::Array => {
            if let Some(items) = &schema.items {
                format!("Array({})", parse_type(items))
            } else {
                "Array".to_string()
            }
        }
        t => t.to_string(),
    }
}

#[test]
pub fn test_basic_conversion() {
    let json_schema_str = r#"
        {
          "$id": "https://example.com/health-record.schema.json",
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "description": "Schema for representing a health record",
          "type": "object",
          "required": ["patientName", "dateOfBirth", "bloodType"],
          "properties": {
            "patientName": {
              "type": "string"
            },
            "dateOfBirth": {
              "type": "string",
              "format": "date"
            },
            "bloodType": {
              "type": "string"
            },
            "allergies": {
              "type": "array",
              "items": {
                "type": "string"
              }
            },
            "conditions": {
              "type": "array",
              "items": {
                "type": "object",
                "required": ["name"],
                "properties": {
                  "name": {
                    "type": "string"
                  },
                  "diagnosisDate": {
                    "type": "string",
                    "format": "date"
                  }
                }
              }
            }
          }
        }"#;

    let json_schema: JsonSchema = serde_json::from_str(json_schema_str).unwrap();
    let fields = convert_schema_to_fields("", &json_schema);
    dbg!(&fields);
    assert_eq!(fields.len(), 7);
    assert_eq!(
        fields
            .iter()
            .find(|f| f.name == "allergies")
            .unwrap()
            .data_type,
        "Array(String)"
    );
    assert!(
        fields
            .iter()
            .find(|f| f.name == "patientName")
            .unwrap()
            .required,
    );
    assert!(
        !fields
            .iter()
            .find(|f| f.name == "allergies")
            .unwrap()
            .required,
    );
    assert_eq!(
        fields
            .iter()
            .find(|f| f.name == "patientName")
            .unwrap()
            .data_type,
        "String"
    );
}
