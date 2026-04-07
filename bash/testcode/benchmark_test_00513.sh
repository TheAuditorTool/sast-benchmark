#!/bin/bash
dump_env_to_logfile() {
    env >> /var/log/app_debug.log
}
