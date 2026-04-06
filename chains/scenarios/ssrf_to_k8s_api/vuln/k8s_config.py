"""Kubernetes API server configuration -- IDENTICAL between vuln/ and safe/.

Represents a K8s cluster where the API server is reachable at an internal
cluster IP. The service account token mounted in the pod grants read access
to secrets and configmaps across namespaces if RBAC is misconfigured.

Chain: attacker -> /render?url=<embed with https://10.0.0.1:443/...> -> K8s secrets

K8s API endpoints exposed:
  https://10.0.0.1:443/api/v1/namespaces/default/secrets
    -> Returns all secrets in the default namespace (DB passwords, API keys)
  https://10.0.0.1:443/api/v1/namespaces/kube-system/configmaps
    -> May contain cluster-level credentials
  https://10.0.0.1:443/apis/rbac.authorization.k8s.io/v1/clusterroles
    -> Enumerates RBAC roles for privilege escalation planning
"""

K8S_API_SERVER = "https://10.0.0.1:443"
K8S_SERVICE_ACCOUNT_TOKEN_PATH = "/var/run/secrets/kubernetes.io/serviceaccount/token"
K8S_CA_CERT_PATH = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt"
K8S_NAMESPACE = "default"
