---
title: Code file
subtitle: Component
---

# Overview

The `CodeFile` component is an alternative to 
[code blocks](/overview/articles#code-blocks) within a document. This will
import an external file as a source code block. This file can be outside the
project root, such as a file located somewhere else in parent project.

Syntax highlighting will be applied based on the file extension.


# Fields


<Field name="file" type="String" required="true">
A path, relative to the project, to the file to render. This file is not 
included in the final site. It is injected directly into the current page.
</Field>

<Field name="collapse" type="bool">
If true the maximum height of the code block will be limited and allow for the 
content to scroll. This is useful for particularly large files.
</Field>

# Example

```HTML
<CodeFile "../somefile.rs" />
```

