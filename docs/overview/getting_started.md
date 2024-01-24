---
title: Getting started
subtitle: Overview
menu_position: -1
---

# Installation

Codex provides binaries for the following platforms:

- Linux
- MacOS (Intel)
- MacOS (Apple Silicon)
- Windows

## Linux/MacOS via shell script 

On Linux Codex can be installed via a installation shell script

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/5Sigma/codex/releases/latest/download/Codex-installer.sh | sh
```


## Windows via Power Shell script

On windows Codex can be installed via Power Shell 

```
irm https://github.com/5Sigma/codex/releases/download/v0.1.0/Codex-installer.ps1 | iex
```

## Windows via installer

An installer is available for windows at [here](https://github.com/5Sigma/codex/releases/latest/download/Codex-x86_64-pc-windows-msvc.zip).


## Binary Downloads

Binary downloads are available in the [Github Release Page](https://github.com/5Sigma/codex/releases/latest).

# Setup a new project

To setup a Codex project within another project run `codex init` and specify 
a project folder. This can be a sub folder of another project. 

```
codex init support/docs
```

This will create the specified path and generate an example codex.yml file at 
the project root. You can [configure](/overview/codex-yml) this file if needed.

# Adding a new article

Articles are written in Markdown, and should have the .md extension.
Articles can be added in the project folder, or within a subfolder. Subfolders
automatically create _groupings_ that are used in the navigation menu. 

**Example**: To setup a new article at the url /overview/getting-started, which
will also setup an _Overview_ group in the navigation. Create a file at:

_project_root_/overview/getting-started.md

This file should have _front matter_ that defines its title, such as:

```markdown
---
title: Getting started
---

# Some tile

Some content
```

# Viewing the project

To view the project run _codex serve_ from the root of the project. This will 
spawn a web server that will serve the project at http://localhost:8080 

