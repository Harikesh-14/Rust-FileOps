# Rust FileOps

Rust File Operations is a command-line tool written in Rust that allows users to perform various file operations such as searching, replacing, creating, and deleting files.

## Features

- **Create:** Create a new file.
- **Append:** Append a word or a sentence in a file.
- **Search:** Search for a word or expression in a file.
- **Replace:** Replace a word or an expression in a file.
- **Delete:** Delete a particular file

## How to Use

1. To use Rust File Operations, you need to have Rust installed on your system. You can install Rust from [here](https://www.rust-lang.org/tools/install).

2. Clone the repository using this command.
  ```
    git clone https://github.com/Harikesh-14/Rust-FileOps.git
  ```
3. To generate the target folder, enter the given below command
  ``` rust
    cargo build
  ```
4. To run the program, use the given below rule
  ``` rust
    cargo run <file operation>
  ```

### Create Operation
To create a file in the main directory, follow these steps:
1. Open a terminal or command prompt.

2. Navigate to the directory where the project is located.

3. Run the following command: 

    ``` rust
   cargo run create -a <filename>
   ``` 

### Search Operation

To search for a word or expression in a file, follow these steps:

1. Open a terminal or command prompt.

2. Navigate to the directory where the project is located.

3. Run the following command:
    ``` rust
    cargo run search <word_to_search> <filename> <is_case_sensitive>
    ```

   Replace `<word_to_search>` with the word or expression you want to search for, `<filename>` with the name of the file to search in, and `<is_case_sensitive>` with `true` if you want the search to be case-sensitive, or `false` otherwise.

   ### For example:
   - This command searches for the word "Rust" in the file "example.txt" with case sensitivity enabled.
       ``` rust
       cargo run search Rust example.txt true
       ```
   - This command searches for the word "Rust" in the file "example.txt" with case sensitivity disabled.
       ``` rust
       cargo run search Rust example.txt false
       ```

## Conclusion

Rust File Operations provides a simple yet powerful way to manipulate files using the Rust programming language. Whether you need to search for specific content in a file or perform more complex file operations, Rust File Operations has you covered.

We hope you find this tool useful for your file manipulation tasks. If you have any feedback, suggestions, or encounter any issues, please feel free to open an issue on our [GitHub repository](https://github.com/Harikesh-14/Rust-FileOps).

### Happy file handling with Rust ❤️

---
