---
title: Project Configuration
subtitle: Overview
---


# Overview

The project as a whole is configured by a _codex.yml_ file, located at the 
project root.

# Fields

<Field name="name" type="String" required="true">
    The display name for the project. This will be displayed in the top header.
</Field>
<Field name="build_path" type="String">
    A path relative to the project path to place compiled files into.
</Field>
<Field name="repo_url" type="String?">
    A url to the project's code repository. If specified a code link will 
    appear in the header.

    To disable this simply omit the field from the config or set it to ~.
</Field>
<Field name="project_url" type="String?">
    A url to the project's main page. If specified a home link will 
    appear in the header.

    To disable this simply omit the field from the config or set it to ~.
</Field>

# Example Configuration

```yml
name: My Project
build_path: dist
repo_url: https://github.com/me/project
project_url: https://myproject.com
```