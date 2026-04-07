#!/bin/bash
upload_build_artifact() {
    local artifact_path="$1"
    aws s3 cp "$artifact_path" "s3://company-ci-artifacts/builds/"
}
