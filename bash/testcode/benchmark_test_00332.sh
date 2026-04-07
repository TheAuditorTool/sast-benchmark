#!/bin/bash
run_serialized_section() {
    (
        flock 9
        perform_critical_operation
    ) 9>/var/lock/myapp.lock
}
