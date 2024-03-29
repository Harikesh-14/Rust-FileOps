# Rust FileOps

Rust File Operations is a command-line tool written in Rust that allows users to perform various file operations such as searching, replacing, creating, and deleting files.

## Features

- **Create file:** Create a new file.
- **Create directory:** Create a new directory/folder. 
- **Append file:** Append a word or a sentence in a file.
- **Search:** Search for a word or expression in a file.
- **Replace:** Replace a word or an expression in a file.
- **Delete file:** Delete a particular file.
- **Delete directory:** Delete a particular directory.

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

### Create a file
To create a file in the main directory, follow these steps:
1. Open a terminal or command prompt.

2. Navigate to the directory where the project is located.

3. Run the following command: 

    ``` rust
   cargo run create -a <filename>
   ``` 

### Create a directory
To create a directory in the main directory, follow these steps:
1. Open a terminal or command prompt.

2. Navigate to the directory where the project is located.

3. Run the following command: 

    ``` rust
   cargo run newdir -a <directory name>
   ``` 

### Appending in a file
To append a file in the main directory, follow these steps:
1. Open a terminal or command prompt.

2. Navigate to the directory where the project is located.

3. Run the following command: 

    ``` rust
   cargo run write -a <filename>
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

### Replace Operation

To replace a word or expression in a file, follow these steps:

1. Open a terminal or command prompt.

2. Navigate to the directory where the project is located.

3. Run the following command:
    ``` rust
    cargo run replace <word_to_replace> <new_word> <filename> <is_case_sensitive>
    ```

   Replace `<word_to_replace>` with the word or expression you want to replace, `<new_word>` with the word or expression you want to replace it with, `<filename>` with the name of the file to perform the replacement in, and `<is_case_sensitive>` with `true` if you want the replacement to be case-sensitive, or `false` otherwise.

   #### For example:
    - This command replaces the word "old" with "new" in the file "example.txt" with case sensitivity enabled.
        ``` rust
        cargo run replace old new example.txt true
        ```
    - This command replaces the word "old" with "new" in the file "example.txt" with case sensitivity disabled.
        ``` rust
        cargo run replace old new example.txt false
        ```

This addition will provide clear instructions on how to use the replace function in your Rust FileOps tool.

### Delete a file

To delete a file, follow these steps:

1. Open a terminal or command prompt.

2. Navigate to the directory where the project is located.

3. Run the following command:
    ``` rust
    cargo run delete -d <filename>
    ```

   Replace `<filename>` with the name of the file you want to delete.

   #### For example:
    - This command deletes the file "example.txt" from the current directory.
        ``` rust
        cargo run delete -d example.txt
        ```

### Delete Directory Operation

To delete a directory, follow these steps:

1. Open a terminal or command prompt.

2. Navigate to the directory where the project is located.

3. Run the following command:
    ``` rust
    cargo run dldir -d <directory_name>
    ```

   Replace `<directory_name>` with the name of the directory you want to delete.

   #### For example:
    - This command deletes the directory "my_directory" from the current directory.
        ``` rust
        cargo run dldir -d my_directory
        ```

## Conclusion

Rust File Operations provides a simple yet powerful way to manipulate files using the Rust programming language. Whether you need to search for specific content in a file or perform more complex file operations, Rust File Operations has you covered.

We hope you find this tool useful for your file manipulation tasks. If you have any feedback, suggestions, or encounter any issues, please feel free to open an issue on our [GitHub repository](https://github.com/Harikesh-14/Rust-FileOps).

### Happy file handling with Rust ❤️

---
