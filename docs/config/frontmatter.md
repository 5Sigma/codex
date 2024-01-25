---
title: Frontmatter
subtitle: Configuration
menu_position: 1
---


# Frontmatter in documents 

Documents should have a block at the beginning of the file which can configure
them. Frontmatter is written in [YAML](http://www.yaml.org).

The general format is:

```YAML
---
title: My Page Title
# other fields
---
...
```

# Fields

<Field name="title" type="String">
This is used as the page title and is rendered into the top header.
</Field>
<Field name="subtitle" type="String">
A subtitle that is rendered smaller and above the main header on the page.
</Field>
<Field name="tags" type="Array(String)" default="[]">
A list of tags that will be rendered in the side bar of the page, below 
the table of contents.
</Field>
<Field name="menu_position" type="Array(String)" default="0">
Sets the position of the page in the navigation menu. A larger number will
be farther down and a smaller number will be closer to the top.

If not specified the value is 0 and negative numbers can be used to prioritize 
groups above default ones.

Pages which have the same position (including pages with unspecified 
positions) will be sorted alphabetically.
</Field>
<Field name="menu_exclude" type="bool" default="false">
If true the page will not be displayed in the navigation menu.
</Field>
