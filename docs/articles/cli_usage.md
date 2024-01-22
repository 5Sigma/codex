---
title: Command Line Overview
subtitle: CLI Usage
tags:
    - CLI
    - Basic Usage
--- 

# Serving a staging environment

The __serve__ command will spawn a web server that will serve the project. 
Pages are rendered live from the files during the request, making it easy to 
see changes.

```sh
codex serve
```

# Building a static site

The **build** command will produce an optimized static site with all the 
project pages. 

When to use whichpermalink
Server load functions are convenient when you need to access data directly from a database or filesystem, or need to use private environment variables.

Universal load functions are useful when you need to fetch data from an external API and don't need private credentials, since SvelteKit can get the data directly from the API rather than going via your server. They are also useful when you need to return something that can't be serialized, such as a Svelte component constructor.

In rare cases, you might need to use both together — for example, you might need to return an instance of a custom class that was initialised with data from your server. When using both, the server load return value is not passed directly to the page, but to the universal load function (as the data property):

        When to use whichpermalink
Server load functions are convenient when you need to access data directly from a database or filesystem, or need to use private environment variables.

Universal load functions are useful when you need to fetch data from an external API and don't need private credentials, since SvelteKit can get the data directly from the API rather than going via your server. They are also useful when you need to return something that can't be serialized, such as a Svelte component constructor.

In rare cases, you might need to use both together — for example, you might need to return an instance of a custom class that was initialised with data from your server. When using both, the server load return value is not passed directly to the page, but to the universal load function (as the data property):
When to use whichpermalink
Server load functions are convenient when you need to access data directly from a database or filesystem, or need to use private environment variables.

Universal load functions are useful when you need to fetch data from an external API and don't need private credentials, since SvelteKit can get the data directly from the API rather than going via your server. They are also useful when you need to return something that can't be serialized, such as a Svelte component constructor.

```sh
codex build
```

# Edgecases

Below are a few configurational options that are available for uncommon situations.

## Specifying a custom project folder

By default codex assumes the current directory contains the project files.
All commands accept a --root-path argument to point to a different location.

```sh
codex --root-path "./docs/" <command>
```

# Generation

Codex can generate boiler plate for various types of documents.


| **Name** | **Description**                                               |
|----------|---------------------------------------------------------------|
| article  | A generic document that is useful for free-form documentation |  
| endpoint | A page that can be used to document an API endpoint           |  

When to use whichpermalink
Server load functions are convenient when you need to access data directly from a database or filesystem, or need to use private environment variables.

Universal load functions are useful when you need to fetch data from an external API and don't need private credentials, since SvelteKit can get the data directly from the API rather than going via your server. They are also useful when you need to return something that can't be serialized, such as a Svelte component constructor.

In rare cases, you might need to use both together — for example, you might need to return an instance of a custom class that was initialised with data from your server. When using both, the server load return value is not passed directly to the page, but to the universal load function (as the data property):

        When to use whichpermalink
Server load functions are convenient when you need to access data directly from a database or filesystem, or need to use private environment variables.

Universal load functions are useful when you need to fetch data from an external API and don't need private credentials, since SvelteKit can get the data directly from the API rather than going via your server. They are also useful when you need to return something that can't be serialized, such as a Svelte component constructor.

In rare cases, you might need to use both together — for example, you might need to return an instance of a custom class that was initialised with data from your server. When using both, the server load return value is not passed directly to the page, but to the universal load function (as the data property):
When to use whichpermalink
Server load functions are convenient when you need to access data directly from a database or filesystem, or need to use private environment variables.

Universal load functions are useful when you need to fetch data from an external API and don't need private credentials, since SvelteKit can get the data directly from the API rather than going via your server. They are also useful when you need to return something that can't be serialized, such as a Svelte component constructor.

In rare cases, you might need to use both together — for example, you might need to return an instance of a custom class that was initialised with data from your server. When using both, the server load return value is not passed directly to the page, but to the universal load function (as the data property):

When to use whichpermalink
Server load functions are convenient when you need to access data directly from a database or filesystem, or need to use private environment variables.

Universal load functions are useful when you need to fetch data from an external API and don't need private credentials, since SvelteKit can get the data directly from the API rather than going via your server. They are also useful when you need to return something that can't be serialized, such as a Svelte component constructor.

In rare cases, you might need to use both together — for example, you might need to return an instance of a custom class that was initialised with data from your server. When using both, the server load return value is not passed directly to the page, but to the universal load function (as the data property):

        When to use whichpermalink
Server load functions are convenient when you need to access data directly from a database or filesystem, or need to use private environment variables.

Universal load functions are useful when you need to fetch data from an external API and don't need private credentials, since SvelteKit can get the data directly from the API rather than going via your server. They are also useful when you need to return something that can't be serialized, such as a Svelte component constructor.

In rare cases, you might need to use both together — for example, you might need to return an instance of a custom class that was initialised with data from your server. When using both, the server load return value is not passed directly to the page, but to the universal load function (as the data property):
When to use whichpermalink
Server load functions are convenient when you need to access data directly from a database or filesystem, or need to use private environment variables.

Universal load functions are useful when you need to fetch data from an external API and don't need private credentials, since SvelteKit can get the data directly from the API rather than going via your server. They are also useful when you need to return something that can't be serialized, such as a Svelte component constructor.

In rare cases, you might need to use both together — for example, you might need to return an instance of a custom class that was initialised with data from your server. When using both, the server load return value is not passed directly to the page, but to the universal load function (as the data property):

# Some header

Some general text

<Alert style="danger"> 
**NOTE**

Some test alert

Some other line **with bold text**

</Alert>
