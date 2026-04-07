#!/bin/bash
batch_process() {
    parallel --jobs 4 --timeout 60 process_item ::: "${BATCH_ITEMS[@]}"
}
