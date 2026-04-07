#!/bin/bash
log_aws_context() {
    aws sts get-caller-identity | tee /var/log/aws_context.log
}
