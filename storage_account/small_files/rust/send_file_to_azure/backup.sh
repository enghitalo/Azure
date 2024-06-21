#!/bin/bash

# This script is used to dump the PostgreSQL database, compress it using zstd, and send it to an Azure storage account.

############## Usage: ##############
# chmod +x backup.sh &&
# cargo build --release &&
# ACCOUNT_NAME='' \
# ACCOUNT_KEY='' \
# CONTAINER_NAME='' \
# POSTGRES_URL='' \
# ./backup.sh
####################################

# Add the following line to the crontab to run the script every 12:35 hours:
# 35 12 * * * ACCOUNT_NAME='' ACCOUNT_KEY='' CONTAINER_NAME='' POSTGRES_URL='' $HOME/Documents/Scripts/a/script.sh &>> $HOME/Documents/Scripts/a/script.log

# Dump the PostgreSQL database and compress it using zstd
timestamp=$(date -u +"%Y-%m-%d_%H:%M:%S")

compressed_file_name="temp_file_$timestamp.sql.zst"

# The variable `compressed_file` represents the path to the compressed file.
# In this case, it is set to "/dev/shm/$compressed_file_name", which is a shared memory (RAM) directory in Linux.
# The compressed file will be stored in this directory during the backup process.
compressed_file="/dev/shm/$compressed_file_name"

echo $timestamp
echo "Dumping the PostgreSQL database and compressing it using zstd..."
time pg_dump --dbname=$POSTGRES_URL | zstd -1 -o "$compressed_file"

# Set the correct BLOB_NAME and FILE_PATH
echo "Sending the compressed file to the Azure storage account..."
time BLOB_NAME="$compressed_file_name" \
FILE_PATH="$compressed_file" \
ACCOUNT_NAME="$ACCOUNT_NAME" \
ACCOUNT_KEY="$ACCOUNT_KEY" \
CONTAINER_NAME="$CONTAINER_NAME" \
POSTGRES_URL="$POSTGRES_URL" \
./target/release/send_file_to_azure

# Remove the temporary compressed file
rm "$compressed_file"