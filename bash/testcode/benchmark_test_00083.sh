#!/bin/bash
run_sandboxed() {
    env -i HOME="$HOME" PATH="/usr/bin:/bin" /usr/bin/date
}
