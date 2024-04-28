<h1 align="center">Tic-Tac-Toe</h1>
<h2 align="center"><a href="https://aymud/tic-tac-toe/">View Live</a></h2>

A Tic-Tac-Toe game application that's using [Slint](https://slint.rs) for the user interface, and Rust for the backend.

The game provides a graphical user interface for two players to take turns marking spaces on a 3x3 grid.
The game ends when a player wins or when the board is filled, resulting in a draw.

<p align="center"><img  src="https://github.com/AMax23/tic-tac-toe/assets/37085550/93d68a7a-7011-4538-9560-5d6d192f6f00" width="40%"></p>

## Deployment

This Tic-Tac-Toe application utilizes WebAssembly (Wasm) to execute the code in web browsers.
This enables Rust code directly in the browser.

WebAssembly Compilation:

- The Rust code is compiled to WebAssembly, producing binary files that can be executed in a web browser.

GitHub Pages Deployment:

- The application is automatically deployed to GitHub Pages using a GitHub Actions workflow.
- The workflow ensures that the latest changes are reflected in the deployed version.

**Note:** While the application can run in the browser, it's important to note that the intended platform is not
necessarily the browser environment.
There may be some bugs when running in the browser, as this mode is primarily for demonstration purposes.
The recommended and ideal way to experience the application is by running it directly on the host's machine without a
browser.


## Running the Application Locally

1. Clone the repository to your local machine
2. Build with cargo. This command compiles the code and generates an executable file.
    ```
    cargo build
    ```
3. Run the application binary
     ```
     cargo run
     ```

## Prerequisites

- Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).  
  Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.
    - `rustc` is the Rust compiler, responsible for translating Rust source code into machine code (executable binaries)
    - `Cargo` is a package manager for Rust
- An Integrated Development Environment (IDE)
    - Visual Studio Code
    - IntelliJ IDEA

## Why Slint & Rust?

This project primarily serves as a practice exercise for developing skills in Rust.

Slint was chosen because it integrates with Rust nicely.
Also, it is a declarative UI framework and provides a concise and expressive syntax for creating user interfaces.

Hakuna Matata! 😊
