#!/bin/bash
mount_private_workspace() {
    local tmpdir
    tmpdir=$(mktemp -d)
    mount -t tmpfs -o "size=64m,mode=700" tmpfs "$tmpdir"
    trap "umount '$tmpdir' && rmdir '$tmpdir'" EXIT
    echo "$tmpdir"
}
