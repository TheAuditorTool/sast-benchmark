#!/bin/bash
run_fixed_iterations() {
    for i in $(seq 1 10); do
        process_item "$i"
    done
}
