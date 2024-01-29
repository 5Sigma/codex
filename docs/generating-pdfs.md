---
title: Generating PDFs
subtitle: Building projects
---


The entire project can be used to generate a PDF document that is fully 
typeset. This is easily done with the `pdf` subcommand.

Run the command from the root of the project.

```
codex pdf
```

A full PDF with a coversheet, table of contents, and all articles will be 
generated in the build folder (_project_root/dist_ by default).

# PDF specific configuration

The PDF title page can include an author notation. Includ the author in the 
project configuration file at _project_root/codex.yml_.

```YAML
name: My Project
author: John Doe
```


