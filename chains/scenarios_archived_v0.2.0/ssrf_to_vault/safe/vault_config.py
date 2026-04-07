"""HashiCorp Vault configuration -- IDENTICAL between vuln/ and safe/.

Represents a Vault server running on the internal network at vault.internal:8200.
The CI/CD pipeline uses Vault's AppRole auth to retrieve secrets at build time.
The Vault root token was used during bootstrap and never revoked, and its
token ID is readable from the environment of the webhook handler process.

Chain: attacker -> /webhook?artifact_url=http://vault.internal:8200/... -> token/secrets

Vault endpoints exposed:
  http://vault.internal:8200/v1/sys/health
    -> Confirms Vault is unsealed and returns version (useful for CVE targeting)
  http://vault.internal:8200/v1/secret/data/production
    -> Returns all production secrets if the handler's token has broad permissions
  http://vault.internal:8200/v1/auth/token/lookup-self
    -> Returns the token's policies and capabilities
"""

VAULT_ADDR = "http://vault.internal:8200"
VAULT_ROLE_ID = "ci-pipeline-role"
VAULT_SECRET_PATH = "secret/data/production"
