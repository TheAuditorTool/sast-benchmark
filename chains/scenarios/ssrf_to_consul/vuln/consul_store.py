"""Consul KV store configuration -- IDENTICAL between vuln/ and safe/.

Represents a Consul agent running on localhost:8500 with ACL tokens disabled.
Consul's KV store holds service configuration including database passwords,
API keys, and TLS certificates injected by the deployment pipeline.

Chain: attacker -> /probe?target=http://localhost:8500/v1/kv/... -> secret dump

Consul endpoints exposed (no auth required):
  http://localhost:8500/v1/kv/?recurse=true
    -> Returns all KV pairs including database credentials
  http://localhost:8500/v1/catalog/services
    -> Enumerates all registered services and their addresses
  http://localhost:8500/v1/acl/tokens
    -> Lists ACL tokens if ACL bootstrap has been run
"""

CONSUL_HTTP_ADDR = "http://localhost:8500"
CONSUL_DATACENTER = "dc1"
CONSUL_KV_PREFIX = "services/myapp"
CONSUL_TOKEN = None
