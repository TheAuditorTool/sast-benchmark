#!/bin/bash
#
# DataForge Infrastructure Setup Script
#
# Wrapper for Terraform that receives variables from Python.
#

set -e

TERRAFORM_DIR="${TERRAFORM_DIR:-./infra}"
ACTION="${1:-plan}"  # plan, apply, destroy

echo "=== DataForge Infrastructure Setup ==="
echo "Action: $ACTION"
echo "Directory: $TERRAFORM_DIR"
echo "Timestamp: $(date -Iseconds)"
echo ""

# vuln-code-snippet start dfg_infra_incomplete_redaction
# Display relevant TF_VAR_* variables (for debugging)
echo "--- Terraform Variables ---"
env | grep "^TF_VAR_" | while read line; do
    var_name=$(echo "$line" | cut -d'=' -f1)
    # Don't print sensitive values
    if [[ "$var_name" == *"password"* ]] || [[ "$var_name" == *"secret"* ]]; then
        echo "$var_name=***REDACTED***"
    else
        echo "$line"  # vuln-code-snippet vuln-line dfg_infra_incomplete_redaction
    fi
done
# vuln-code-snippet end dfg_infra_incomplete_redaction
echo ""

# Change to terraform directory
cd "$TERRAFORM_DIR"

# Initialize Terraform
echo "--- Initializing Terraform ---"
terraform init -input=false

# Validate configuration
echo "--- Validating Configuration ---"
terraform validate

# Run requested action
case "$ACTION" in
    plan)
        echo "--- Running Terraform Plan ---"
        terraform plan -out=tfplan
        ;;

    apply)
        echo "--- Running Terraform Apply ---"
        # Check if plan exists
        if [ -f "tfplan" ]; then
            terraform apply -auto-approve tfplan
        else
            terraform apply -auto-approve
        fi
        ;;

    destroy)
        echo "--- Running Terraform Destroy ---"
        echo "WARNING: This will destroy all infrastructure!"
        terraform destroy -auto-approve
        ;;

    output)
        echo "--- Terraform Outputs ---"
        terraform output -json
        ;;

    *)
        echo "ERROR: Unknown action: $ACTION"
        echo "Usage: $0 [plan|apply|destroy|output]"
        exit 1
        ;;
esac

echo ""
echo "=== Infrastructure Setup Complete ==="
exit 0
