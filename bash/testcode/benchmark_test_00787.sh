#!/bin/bash
start_app_server() {
    local jar_path="$1"
    JAVA_OPTS="-Dcom.sun.net.ssl.checkRevocation=false" java -jar "$jar_path"
}
