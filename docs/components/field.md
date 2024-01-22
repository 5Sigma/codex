---
title: Fields
subtitle: Component
---

# Usage

The field component is used to describe a typed or untyped field of an object. 
This can be used for class definitions, interchange format documentation such 
as yaml or json, or other similar uses.

# Properties

<Field name="name" type="String" required="true">
The name of the field
</Field>

<Field name="type" type="String">
The type name of the field. This can be omitted if not applicable.
</Field>

# Example

```html
<Field name="type" type="String">
The type name of the field. This can be omitted if not applicable.
</Field>
```

