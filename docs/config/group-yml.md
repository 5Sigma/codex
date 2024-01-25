---
title: Group Configuration
subtitle: Configuration
---


# Overview

Groups are sub paths inside the project root and are used to group documents. 
By default the directory name is used to generate the name for the group and 
navigation is built based on these paths. To override some of the default 
behavior for a group a group.yml file can be added to it.

# Fields

<Field name="name" type="String">
    The display name for the group. This will be used in the navigation. 
    If not specified the directory name is used.
</Field>
<Field name="menu_position" type="Integer">
Sets the position of the group in the navigation menu. A larger number will
be farther down and a smaller number will be closer to the top.
If not specified the value is 0 and negative numbers can be used to prioritize 
groups above default ones.

Groups which have the same position (including groups with unspecified 
positions) will be sorted alphabetically.
</Field>
<Field name="menu_exclude" type="bool">
If true the group will be hidden from the navigation. It will still be 
built and available via direct links. 
</Field>

# Example Configuration

```YAML
name: Overview
menu_position: -1
```
