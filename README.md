# Simulated Shell in Rust

This project implements a simple shell in Rust that interprets user-provided commands via the command line, executing some basic internal commands and also running operating system programs. The shell simulates the behavior of a Unix terminal, like Bash, and handles environment variables, command-line arguments, and character escapes.

## Features

The implemented shell has the following features:

1. **Internal Commands:**
    - **exit**: Exits the shell, with the option to specify an exit code.
    - **echo**: Prints the provided arguments.
    - **type**: Tells whether a command is an internal shell command or an executable program.
    - **pwd**: Prints the current working directory.
    - **cd**: Changes the current working directory. If no argument is provided, it changes to the user's home directory.

2. **Environment Variable Handling:**
    The shell handles environment variables inside double quotes, substituting the variable value (e.g., `$HOME`).

3. **Character Escaping:**
    - The shell allows escaping special characters like spaces, quotes (`"`, `'`), and backslashes (`\`).
    - Supports escaping characters inside both single and double quotes.

4. **Argument Handling:**
    The shell processes arguments passed to commands, including handling spaces and strings with quotes.

## How to Use

1. Clone the repository:

    ```bash
    git clone https://github.com/username/repo.git
    cd repo
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Run the shell:

    ```bash
    cargo run
    ```

4. Once running, the shell will wait for commands in the command line. You can enter commands like the following:

    - `echo Hello, world!` — Prints `Hello, world!`.
    - `pwd` — Displays the current directory.
    - `cd /path/to/directory` — Changes to the specified directory.
    - `type echo` — Tells that `echo` is an internal shell command.
    - `exit 0` — Exits the shell with exit code `0`.

## Code Structure

The code is written in Rust and consists of:

- **Enum `States`**: Represents the different parsing states (such as alphanumeric, single quote, double quote, escape, and environment variable).
- **Function `parse_cli`**: Responsible for interpreting the user's input and splitting the command line into commands and arguments, handling cases for quotes, environment variables, and character escaping.
- **Function `main`**: Runs the shell, processes user commands, and calls internal functions or executes system programs.

## How the Parsing Works

The command parsing process in the shell is handled based on the following states:

1. **Alphanumeric**: Handles regular characters and words.
2. **Single Quote**: Handles text inside single quotes, where characters are treated literally.
3. **Double Quote**: Handles text inside double quotes, allowing environment variable substitution.
4. **Escape**: Handles escaped characters like `\n`, `\"`, or `\\`.
5. **Environment Variable**: Handles environment variables when found inside double quotes.

## Contributing

Contributions are welcome! To contribute:

1. Fork this repository.
2. Create a branch for your change (`git checkout -b my-change`).
3. Make your changes and commit them (`git commit -am 'Adds new feature'`).
4. Push to the remote repository (`git push origin my-change`).
5. Open a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).
