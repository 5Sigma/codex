---
title: Project Structure
---

# Creating a new project

The easiest way to generate a new project is with the _init_ command using 
the CLI. This will generate a [codex.yml](/overview/codex-yml) in the 
project path and an example article.

```
codex init 
```

<Alert style="info">
By default, the this assumes the current path is the root of the project.
the _--root_path_ flag can be used to specify an alternate location.
</Alert>

# Layout of the project

The project layout is meant to be fairly free form. Sub paths are used to group
articles. By default, the directory name is used to name the grouping and this 
is used to build the navigation menu. For simple situations, nothing needs to
be done other than logically organize articles into folders.  

To customize the groups, a [group.yml](/overview/group-yml) file can be placed 
inside a folder. This will allow configuring things like a custom display name, 
whether it should appear in the navigation menu, and if so, in what order.

```
root folder
  - codex.yml
  - grouping folder
     -  article.md
    grouping folder
     -  group.yml
     -  article.md
```
