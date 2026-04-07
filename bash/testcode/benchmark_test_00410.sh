#!/bin/bash
purge_temp_files_quoted() {
    for file in "${FILE_LIST[@]}"; do
        rm "$file"
    done
}
