---
title: Fields
subtitle: Component
---

# Usage

The field component is used to describe a typed or non-typed field of an object. 
This can be used for class definitions, interchange format documentation such 
as YAML or JSON, or other similar uses.

# Properties

<Field name="name" type="String" required="true">
The name of the field
</Field>

<Field name="type" type="String">
The type name of the field. This can be omitted if not applicable.
</Field>

<Field name="type_link" type="String">
If present the type will be converted into a link to the url specified here.
</Field>

<Field name="deprecated" type="Bool">
If true the field will be marked with a deprecation warning and badge. 
</Field>

# Example

```HTML
<Field 
    name="type" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
The type name of the field. This can be omitted if not applicable.
</Field>
```
<Field 
    name="basic_field" 
    type="String" 
    type_link="https://en.wikipedia.org/wiki/String_(computer_science)">
    Here is an example field with a type link
</Field>


```HTML
<Field 
    name="type" 
    type="String" 
    deprecated="true">
    Here is an example field that has been deprecated.
</Field>
```
<Field 
    name="deprecated_field" 
    type="String" 
    deprecated="true">
    Here is an example field that has been deprecated.
</Field>


