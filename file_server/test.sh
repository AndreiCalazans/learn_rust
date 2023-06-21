#!/bin/bash

file_path="./test.txt"
filename="file_name.txt"
upload_url="http://localhost:8080/upload"

# Check if the file exists
if [ ! -f "$file_path" ]; then
  echo "File does not exist: $file_path"
  exit 1
fi

# Make the file upload request
curl --form "file=@$file_path;filename=$filename" $upload_url

echo "ls in ./uploads"
ls -l ./uploads
cat ./uploads/$filename
