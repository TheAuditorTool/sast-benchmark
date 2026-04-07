#!/bin/bash
batch_process() {
    local processor="$1"
    find /var/queue -type f | xargs -I{} sh -c "$processor {}"
}
