#!/bin/bash
create_restricted_data_dir() {
    install -d -m 750 /var/app/data
}
