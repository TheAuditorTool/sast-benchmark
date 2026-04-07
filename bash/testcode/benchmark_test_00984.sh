#!/bin/bash
download_package() {
    local pkg_name="$1"
    wget -q "https://packages.internal/pool/${pkg_name}.deb"
}
