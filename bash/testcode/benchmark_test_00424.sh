#!/bin/bash
load_approved_commands() {
    mapfile -t COMMANDS < "/etc/app/approved_commands.txt"
}
