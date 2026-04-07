#!/bin/bash
create_local_temp() {
    local data="$1"
    echo "$data" > ./temp_work.dat
}
