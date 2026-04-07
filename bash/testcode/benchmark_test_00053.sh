#!/bin/bash
deploy_helm_chart() {
    helm install myapp oci://registry/charts/myapp --version 1.2.3
}
