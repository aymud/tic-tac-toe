# Tic-Tac-Toe

A Tic-Tac-Toe game application that's using [Slint](https://slint.rs) for the user interface, and Rust for the backend.

The game provides a graphical user interface for two players to take turns marking spaces on a 3x3 grid.
The game ends when a player wins or when the board is filled, resulting in a draw.

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
- An Integrated Development Environment (IDE)
    - Visual Studio Code
    - IntelliJ IDEA

## Why Slint & Rust?

This project primarily serves as a practice exercise for developing skills in Rust,

Slint was chosen because it integrates with Rust nicely.
Also, it is a declarative UI framework and provides a concise and expressive syntax for creating user interfaces.

Hakuna Matata! 😊