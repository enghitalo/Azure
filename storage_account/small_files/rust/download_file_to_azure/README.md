# Send File to Azure

This repository contains a Rust program that demonstrates how to send a file to Azure Blob Storage.

## Prerequisites

Before running the program, make sure you have the following:

- Rust installed on your machine
- Azure Blob Storage account
- Azure Storage connection string

## Installation

1. Clone this repository to your local machine.
2. Open a terminal and navigate to the project directory.
3. Run the following command to build the program:

   ```sh
   cargo build --release
   ```

## Usage

To use the program, follow these steps:

1. Set environment variables to your Azure Storage connection string.

   ```sh
       ACCOUNT_NAME=''
       ACCOUNT_KEY=''
       CONTAINER_NAME=''
       BLOB_NAME=''
       FILE_PATH=''
   ```

2. Run the program using the following command:

   ```sh
       cargo run
   ```

   For example, you can run the program with the following command:

   ```sh
   cargo build --release &&
    ACCOUNT_NAME='' \
    ACCOUNT_KEY='' \
    CONTAINER_NAME='' \
    BLOB_NAME='' \
    DOWNLOAD_PATH='' \
    cargo run
   ```

   Make sure to replace `$FILE_PATH`, `$CONTAINER_NAME`, and `$BLOB_NAME` with the actual values.

3. The program will upload the file to Azure Blob Storage and display the URL of the uploaded blob.

## Environment Variables

The following environment variables need to be set:

You can set the environment variables for the current session in a Linux terminal by running the following command:

```sh
cargo build --release &&
ACCOUNT_NAME='' \
ACCOUNT_KEY='' \
CONTAINER_NAME='' \
BLOB_NAME='' \
FILE_PATH='' \
./target/release/send_file_to_azure
```

Here are a few more examples of setting environment variables for the current session:

- Set a single environment variable:

  ```sh
  export VARIABLE_NAME="value"
  ```

- Set multiple environment variables:

  ```sh
  export VARIABLE1_NAME="value1" VARIABLE2_NAME="value2" VARIABLE3_NAME="value3"
  ```

- Set environment variables with special characters:
  ```sh
  export VARIABLE_NAME="value with spaces" VARIABLE2_NAME="value with 'quotes'"
  ```

Remember that these environment variables will only be available for the current session and will not persist after the terminal is closed.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
