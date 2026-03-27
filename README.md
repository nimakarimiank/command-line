# Rust Command Line Search Tool (Mini-Grep)

## Overview
This project is a custom, lightweight implementation of the classic command-line search tool `grep` (globally search a regular expression and print). It is built entirely in Rust and demonstrates the language's speed, safety, and excellent cross-platform support. 

Given a file path and a search string, this tool reads the file, identifies all lines containing the target string, and prints those lines to the terminal.

## Features
* **Fast Text Searching:** Quickly scans text files for specific string patterns.
* **Environment Variable Configuration:** Supports toggling case-insensitive searching via environment variables.
* **Proper Error Handling:** Gracefully handles missing files or insufficient arguments using Rust's `Result` type.
* **Standard Streams:** Routes successful output to Standard Output (stdout) and error messages to Standard Error (stderr), allowing for seamless integration with other terminal tools and file redirection.

## Getting Started

### Prerequisites
You will need to have Rust and Cargo installed on your machine.
* [Install Rust](https://www.rust-lang.org/tools/install)

### Installation
1. Clone the repository:
   ```bash
   git clone [https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git](https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git)