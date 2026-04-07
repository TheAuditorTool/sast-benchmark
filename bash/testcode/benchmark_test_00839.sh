#!/bin/bash
write_pidfile() {
    echo $$ > /var/run/app.pid
    chmod 666 /var/run/app.pid
}
