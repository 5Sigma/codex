---
title: Csv table
subtitle: Components
---


# Overview

The `CsvTable` component can render an external CSV file into a table, with
optional headers.


# Properties

<Field name="file" type="String" required="true">
A path relative to the project root to a CSV file to render. This file needs 
to be present during build, but does not get moved into the final site.
</Field>

<Field name="headers" type="Boolean" default="true">
If true the first row of the file is parsed as a header row and a table header 
is generated for it.
</Field>


# Example

```HTML
<CsvTable file="../test/fixture/other/test_data.csv"/>
```

<CsvTable file="../test/fixture/other/test_data.csv" headers="true" />
