---
title: Generating LaTeX
subtitle: Building projects
---

Similar to PDFs a raw tex file can be generated for the porject.

```
codex latex
```

A full PDF with a coversheet, table of contents, and all articles will be 
generated in the build folder (_project_root/dist_ by default).

# PDF specific configuration

The title page can include an author notation. Include the author in the 
project configuration file at _project_root/codex.yml_.

```YAML
name: My Project
author: John Doe
```

