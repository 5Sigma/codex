---
title: Alert Panel
subtitle: Component
---

# Usage

The alert component can be used to display various alert panels.

# Properties

<Field name="title" type="String" required="true">
A heading to be placed inside the alert panel.
</Field>
<Field name="style" type="Enum">
A visual theme for the alert. Valid options are:
- danger
- primary
- info
- warning
</Field>

# Examples

## A basic alert

```HTML
<Alert style="danger">
An example alert message
</Alert>
```

<Alert style="danger">
An Example alert message
</Alert>


## An alert with a heading

```HTML
<Alert style="warn" title="Be careful doing this">
A reason why doing *that* is bad.
</Alert>
```

<Alert style="warning" title="Be careful doing this">
A reason why doing *that* is bad.
</Alert>

## Complex markdown inside alerts

```HTML
<Alert style="info" title="More information">
Here are some things to be aware of
- Item number 1 
- **Item** number two
</Alert>
```

<Alert style="info" title="More information">
Here are some things to be aware of
- Item number 1 
- **Item** number two
</Alert>
