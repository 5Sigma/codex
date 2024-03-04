---
title: Generating LaTeX
subtitle: Building projects
---

As an alternative to static HTML generation, Codex can generate a LaTeX for the project. This can then be turned into a typesetted PDF with tools like Tectonic, or Xelatex.

```
codex latex
```

The LaTeX output contains the styling for the same components, a coversheet, table of contents, and all articles.
The LaTeX file will generate in the build folder (_project_root/dist_ by default).

# LaTeX specific configuration

The title page can include an author notation. Include the author in the 
project configuration file at _project_root/codex.yml_.

```YAML
name: My Project
author: John Doe
```

