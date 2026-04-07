#!/bin/bash
process_items_bounded() {
    xargs -P 4 -n 1 process_item
}
