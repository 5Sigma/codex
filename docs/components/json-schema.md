---
title: JSON Schema
subtitle: Components
--- 

# Generating fields

The `JsonSchemaFields` component can analyze a _jsonschema_ file and build a 
field listing from it.

Nested field structures can be rendered and markdown syntax inside the 
description field will be rendered.

This component supports a subset of the _jsonschema_ properties:

- It will apply required flags to the field
- It will add format information to the description 
- It will apply type definitions to the field
- It will will generate nested fields with dot notation names


## Contents of medical.json

<CodeFile file="cookbook/medical.json"/>

## Component usage

```HTML
<JsonSchemaFields file="cookbook/medical.json"/>
```

<JsonSchemaFields file="cookbook/medical.json"/>

# Generating an example

An example block can also be generated automatically from a schema file.

```HTML
<JsonSchemaExample file="cookbook/medical.json"/>
```

<JsonSchemaExample file="cookbook/medical.json"/>
