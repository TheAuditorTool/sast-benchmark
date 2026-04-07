#!/bin/bash
create_test_image() {
    dd if=/dev/zero bs=1M count=100 of=/tmp/test.img
}
