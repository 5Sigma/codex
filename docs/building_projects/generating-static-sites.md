---
title: Static site generation
subtitle: Building projects
---

To build a static site from the project use the `build` subcommand from the 
CLI.

From the root of the project run:

```
codex build
```


# Site structure

The static site is generated in the build folder, by default this is ./dist 
relative to the project root.

Each markdown document generates a HTML page using the same path structure as
the project. These are generated as _index.html_ files to give nice looking 
URLs.

A markdown file located at _project_root/articles/getting-started.md_ will 
generate an HTML file at _build_folder/articles/getting-started/index.html_ 

All files located in the static folder at _project_root/static_ will be copied 
directly to the build folder.


# Automatic assets

Several assets will be automatically included in the final build. 

- A custom CSS file for styling the site.
- Bootstrap CSS 
- Bootstrap JS 
- A FontAwesome CSS/JS/Font with a small number of select icons.
- The overpass font


# Base URL

In order to host the static site within a subfolder you can set the `base_url`
in the project configuration. This will change all links in the navigation and 
all articles to point to the new base path.
