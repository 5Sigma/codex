---
title: Github pages
subtitle: Pipeline Deployment
---

# Assumptions

This guide has several assumptions that may alter your configuration slightly,
depending on your specific configuration.

- The documentation project folder is located at _docs/_
- You are hosting on GitHub pages
- You are not using a custom domain name which means the finalized page is 
located at https://_yourname_.github.io/_your_project_

# Setting up the project

Setup the project as normal using _docs/ as the project root path. From the 
root of the parent project run:

```
codex init docs
```

# Setup a base URL

Because the default URL format of GitHub pages is /project/ we need a base URL
so relative URLs are prefixed with the project name correctly.

Edit docs/codex.yml:

```YAML
name: My Project Name
base_url: repo_name
```

# Build pipeline 

The build pipeline will perform the following steps:

- Download the latest Codex binary
- Build the static site
- Upload the static site as an artifact
- Publish to GitHub pages


```YAML
name: Publish Documentation

on:
  push:
    branches:
    - main

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Build 
        run: |
          curl --proto '=https' --tlsv1.2 -LsSf https://github.com/5Sigma/codex/releases/latest/download/Codex-installer.sh | sh
          codex -r docs build
      - name: Upload Pages artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: docs/dist

  deploy:
      needs: build
      permissions:
        pages: write
        id-token: write

      environment:
        name: github-pages
        url: ${{ steps.deployment.outputs.page_url }}

      runs-on: ubuntu-latest
      steps:
        - name: Deploy to GitHub Pages
          id: deployment
          uses: actions/deploy-pages@v4


```




