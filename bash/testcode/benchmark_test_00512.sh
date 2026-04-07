#!/bin/bash
create_restricted_lockfile() {
    install -m 600 /dev/null /var/run/app.lock
}
