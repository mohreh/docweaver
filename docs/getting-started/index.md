---
title: Getting Started
order: 1
icon: iconsax/linear/play-circle
---

## Getting Started

### Prerequisites
- Rust and Cargo installed
- TailwindCSS CLI installed

### Installation
1. Clone the repository:
```sh
git clone https://github.com/mohreh/docweaver.git
cd docweaver
```

2. Build the repository:
```sh
cargo build --release
```

3. Run the project:
```sh
cargo run --release
```


### Adding Documentation

1. Create Markdown Files: Add your Markdown files to the docs/ directory. DocWeaver will automatically render these files and serve them as documentation.
2. Update Configuration: Edit the config.yml file to customize the application settings, such as title, description, quick links, and features.


### Customizing Styles

DocWeaver supports custom CSS styling. You can tailor the appearance and branding of your documentation site by editing the templates/assets/main.scss file and compiling it using TailwindCSS.
Serving Static Files

DocWeaver allows you to serve static files, such as CSS stylesheets, JavaScript files, and images, alongside your Markdown content. Place these files in the assets/ directory.
Developer-Friendly Features

Built with Rust, DocWeaver takes advantage of Rust's performance, safety, and concurrency features, ensuring a reliable and efficient documentation experience.
