---
title: What is Codex
---

# Codex

Codex is an opinionated, but flexible, static site generator; built 
specifically for technical documentation.

Its primary goal is minimal configuration, quick deployment, and easy 
modification. In order to do this, it is very opinionated by default. However,
almost all parts of the system can be ejected from defaults and overwritten. 
This gives the ability to setup quickly with a sane, and recommended, 
configuration; or reconfigure everything to suite additional needs.   

# Philosophy 

## Store documentation with the project 

Codex uses simple markdown files that can be stored alongside the actual 
project. This makes it much easier to keep documentation in sync with the 
project, doesn't require another storage system, and allows it to benefit from 
the project's version control.

## Portable

Codex is meant to build & publish documentation via CI/CD. It is shipped as a 
single binary that is less than 5mb in size. This makes it easy to download and 
use within a pipeline.

## Minimal configuration

Codex is meant to be able to setup and modify quickly without worrying to much
about configuration and styling. It tries to provide a default configuration 
that will work for most use cases with as little configuration as possible.

# Markdown

Codex uses markdown for article writing. Markdown has several pros and cons:

## Benefits of markdown
- Easy to write without any specialized software
- Non technical and obvious even in a basic text editor or output
- Easy to read without any specialized software

## Cons of markdown
- Extremely simple formatting, which is insufficient for technical 
documentation
- No separation of formatting and semantic meaning.

## Complicating markdown for technical writing

There are some additions to markdown to help mitigate its limited styling like
[CommonMark](https://commonmark.org/) and [Github Flavored Markdown](https://github.github.com/gfm/). These are helpful but don't go far 
enough.

Codex makes a choice, good or bad, to add additional complexity to markdown to
help facilitate more technically structured writing. It still maintains 
backwards compatibility with markdown, but adds an additional 
_Component Architecture_ based on JSX.

# Frequenty Asked Questions

> Can the color scheme, themes, and other visual styling by modified.

Codex has no theme engine. It has a single default visual style. There are no 
configurational options for altering the visual style of the built pages.

However, all the templates that are used to build the final site can be 
overwritten, effectively making the entire site 100% customizable.

> Does codex support writing formats other than Markdown, such as OrgMode

Currently, it does not.

> Can custom JSX components be used 

Yes additional components can be written using a handlebars syntax for the 
component temlate and used in markdown with JSX syntax.

> What kind of documentation is codex suited for.  

Codex can be used for most technical documentation but the use cases that are 
front most from the perspective of it's development are:

- API Documentation
- Object/Class documentation
- Project overview and usage documentation
 
