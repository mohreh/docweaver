---
title: Project Structure
order: 3
icon: iconsax/linear/tree
---

# Project Structure

## Overview
The DocWeaver project is structured to separate concerns and make it easy to manage different aspects of the application. Below is a detailed breakdown of the project structure.

## Root Directory
- `assets/`: Contains static assets like favicon, header image, and compiled CSS.
- `build.rs`: Script to compile TailwindCSS.
- `Cargo.lock`: Cargo lock file.
- `Cargo.toml`: Cargo configuration file.
- `config.yml`: Application configuration file.
- `Dockerfile`: Docker configuration file.
- `docs/`: Directory containing Markdown documentation files.
- `src/`: Source code directory.
- `target/`: Compiled output directory.
- `templates/`: Directory containing HTML templates.

## Assets Directory
- `favicon.ico`: Favicon for the application.
- `header.png`: Header image for the application.
- `styles/main.css`: Compiled TailwindCSS file.

## Templates Directory
- `assets/`: Contains icons and main SCSS file.
- `base.html`: Base HTML template.
- `common/`: Common HTML components.
  - `head.html`: Head section of the HTML.
  - `nav.html`: Navigation bar.
- `doc.html`: Template for documentation pages.
- `index.html`: Homepage template.

## Configuration File
The `config.yml` file allows you to customize the application settings, such as port, environment, title, description, quick links, direction, and features.

## Build Script
The `build.rs` script compiles TailwindCSS using the following command:
```rust
Command::new("tailwindcss")
    .args([
        "-i",
        "./templates/assets/main.scss",
        "-o",
        "./assets/styles/main.css",
        "-m",
    ])
    .status()
    .unwrap();
