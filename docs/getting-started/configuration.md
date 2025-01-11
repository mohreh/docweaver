---
title: Configuration
order: 2
icon: iconsax/linear/setting-2
---

# Configuration

## Overview
The `config.yml` file allows you to customize various aspects of the DocWeaver application. Below is a detailed breakdown of the configuration options.

## Configuration Options

### General Settings
- `port`: The port on which the application will run.
- `app_environment`: The environment in which the application is running (e.g., production, development).

### Application Settings
- `head_link`: Array of head link objects. Each object can have the following properties:
  - `rel`: The relationship of the link.
  - `href`: The URL of the link.
  - `type`: The type of the link.
- `title`: The title of the application.
- `description`: A brief description of the application.
- `quick_links`: Array of quick link objects. Each object can have the following properties:
  - `name`: The name of the link.
  - `href`: The URL of the link.
  - `icon`: The icon of the link.
- `direction`: The text direction of the application (e.g., ltr, rtl).

### Features
- `features`: Array of feature objects. Each object can have the following properties:
  - `title`: The title of the feature.
  - `description`: A brief description of the feature.
  - `icon`: The icon of the feature.

### Navigation and Sidebar
- `nav`: Array of navigation objects (optional). Each object can have the following properties:
  - `title`: The title of the navigation item.
  - `path`: The path to the Markdown file.
- `sidebar`: Array of sidebar objects (optional). Each object can have the following properties:
  - `title`: The title of the sidebar item.
  - `path`: The path to the Markdown file.

## Example Configuration
```yaml
port: 8000
app_environment: production
application:
  head_link:
    - rel: icon
      href: assets/favicon.ico
      type: image/x-icon
  title: DocWeaver
  description: A Seamless Markdown Documentation Experience
  quick_links:
    - name: Getting Started
      href: /getting-started
      icon: iconsax/linear/arrow-right-1.svg
    - name: GitHub
      href: https://github.com/mohreh/docweaver
  direction: ltr
  features:
    - title: Seamless Markdown Rendering
      description: DocWeave allows you to render and serve Markdown files on-the-fly, providing an interactive and up-to-date documentation experience. Your content is dynamically converted to HTML, ensuring that any changes to your Markdown files are immediately reflected.
      icon: iconsax/linear/document-text-1.svg
    - title: Hierarchical Navigation Structure
      description: DocWeave automatically generates a navigation sidebar based on the directory structure of your Markdown files. This hierarchical organization makes it easy for users to navigate through different sections and subsections, providing a smooth and intuitive browsing experience.
      icon: iconsax/linear/category-2.svg
    - title: Custom Styling
      description: DocWeave supports custom CSS styling, allowing you to tailor the appearance and branding of your documentation site. You can easily adapt the look and feel to match your project's design or personal preferences.
      icon: iconsax/linear/brush-1.svg
    - title: Static File Serving
      description: DocWeave allows you to serve static files, such as CSS stylesheets, JavaScript files, and images, alongside your Markdown content, providing a complete and self-contained documentation website.
      icon: iconsax/linear/document-cloud.svg
    - title: Developer-Friendly
      description: Built with Rust, DocWeave is a developer-friendly application that takes advantage of Rust's performance, safety, and concurrency features, ensuring a reliable and efficient documentation experience.
      icon: iconsax/linear/programming-arrow.svg
