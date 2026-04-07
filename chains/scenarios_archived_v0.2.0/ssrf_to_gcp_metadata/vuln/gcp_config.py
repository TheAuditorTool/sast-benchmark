"""GCP metadata service configuration -- IDENTICAL between vuln/ and safe/.

On Google Cloud Platform, every VM can reach the instance metadata server
at the link-local address 169.254.169.254. Unlike AWS, GCP metadata v1 does
not require the X-Forwarded-For header to be absent, but it does require
the Metadata-Flavor: Google header -- which urllib will happily send if
the attacker includes it in a crafted URL or via request headers.

Chain: attacker -> /tiles?url=http://169.254.169.254/... -> service account token

GCP metadata endpoints:
  http://169.254.169.254/computeMetadata/v1/instance/service-accounts/default/token
    -> Returns an OAuth2 access token for the VM's service account
  http://169.254.169.254/computeMetadata/v1/project/project-id
    -> Returns the GCP project ID
  http://169.254.169.254/computeMetadata/v1/instance/attributes/ssh-keys
    -> Lists SSH public keys added via OS Login or project metadata
"""

GCP_METADATA_ENDPOINT = "http://169.254.169.254/computeMetadata/v1"
GCP_METADATA_FLAVOR_HEADER = "Google"
GCP_PROJECT_ID = "my-production-project"
