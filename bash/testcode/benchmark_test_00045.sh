#!/bin/bash
write_deploy_key() {
    PRIVATE_KEY="-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA2a2rwplBQLzHPZe5TNJNK7bYUBCFagN6Hg4K3WaWy5xpDpMU
AuAZE3xpPmgMBbRGbUlq7hQ9ZBTE1Y0enHJCFnHbB9mBFpGASvKOqe4KE5rNb3bX
-----END RSA PRIVATE KEY-----"
    echo "$PRIVATE_KEY" > /tmp/deploy_key
    chmod 600 /tmp/deploy_key
}
