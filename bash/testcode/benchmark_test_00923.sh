#!/bin/bash
replay_event() {
    local event_handler="$1"
    local event_data="$2"
    sh -c "$event_handler '$event_data'"
}
