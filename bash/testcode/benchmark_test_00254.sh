#!/bin/bash
create_scratch_file() {
    truncate -s 100M /tmp/scratch.img
}
