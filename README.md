# Byte-Bite

![](https://img.shields.io/badge/license-GPL%203.0-green)
![](https://img.shields.io/badge/Powered%20By-Rust-blue)
![](https://img.shields.io/badge/Crates.io-0.1.4-blue)

Take a bite out of the news and updates with ByteBite, the bite-sized RSS feed reader that delivers all the essential news in a pocket-size format.

<p align="center">
<img
  src="/logos/jpg/logo-black.jpg"
  title="ByteBite"
  width="25%"
  height="25%">
</p>

https://user-images.githubusercontent.com/62032096/228919017-3f582f1b-da9e-449a-a732-f0f1d3b81a97.mp4


# Key Features:
- Enables users to add/remove RSS feeds
- Incremental refresh for RSS articles
- Help menu provided to help users with keyboard navigation

# Getting Started:
Visit the [Byte-Bite official repository](https://github.com/oss-rust-github-io/byte_bite) to download and install the application on the host machine.

# Configuration
- RSS feed information is stored in "data/rss_db.json" file
- RSS articles information is stored in "data/article_db.json" file
- Logging configuration information is stored in "logging_config.yaml" file
- Error codes are stored and maintained in "src/error_db.rs" file

# Keybindings
- a --> Add new RSS feed url
- d --> Delete existing RSS feed
- r --> Refresh articles for RSS feed
- h --> Open help menu
- q --> Exit the application
- page-up / page-down --> Navigate through list of RSS feeds
- arrow-up / arrow-down --> Navigate through list of articles in each RSS feed
- esc --> Exit RSS add option / Exit popup windows

# Roadmap
The goal is to eventually evolve and package the application for all operating systems.
