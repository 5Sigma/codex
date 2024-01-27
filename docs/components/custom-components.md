---
title: Custom components
subtitle: Components
menu_position: 10
---


# Anatomy of a component

Components are built using html files that contain handlebar syntax. This 
keeps them very straight forward and easy to deal with. It also means they are 
not able to do any kind of advanced logic you might find in a web framework.

They are intended to provide structure and styling in places where markdown 
fails, not as dynamic web components like React and others.

Components are stored in a components path inside special *_internal* 
folder located at the project root.

# Creating a hello component

Inside a new project create a file in 
_project_root_/_internal/components/hello.html

The component code is simple html we will use:

```HTML
<h4>Hello, {{name}}</h4>
```

Any properties passed to the component are provided as variables to the 
component template and can be used with Handlebar syntax.

--- 

Now create a document at _project_root_/hello_page.md to use the component

```Markdown
# Component test
This is a test of the hello component:
<Hello name="Alice" />
```

# Overriding existing components

Default components can also be overridden simply by redefining them in the 
project. Components locally defined inside _internal/components will take 
precedence over default ones.

For instance to override the alert component we could create 
_project_root_/_internal/alert.html with:

```HTML
<div class="my-alert">
    {{#if title}}
        <div class="my-title">{{title}}</div>
    {{/if}}
    {{{children}}}
</div>
```


# Component children 

Components may have children which allows them to wrap other components or 
markdown blocks. The alert component above uses the children variable.
A component with children is used like this:

```Markdown
<Alert title="My Title">
    # A header

    Some more *markdown* code
    <AnotherComponent />
</Alert>
```

<Alert style="light">
Note the three bracers used for \{\{\{children\}\}\} in the overriding example. This
will output the result without HTML escaping.
</Alert>


