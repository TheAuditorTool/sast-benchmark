#!/bin/bash
dump_debug_info() {
    env | tee /tmp/debug_env.log
}
