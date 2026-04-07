#!/bin/bash
purge_temp_files() {
    for file in $FILE_LIST; do
        rm $file
    done
}
