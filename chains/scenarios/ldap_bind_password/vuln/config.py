"""Application configuration for ldap_bind_password scenario -- VULNERABLE variant.

The LDAP service account bind password is hardcoded. An attacker can
use it to authenticate to the LDAP server as the service account,
enumerate directory contents, or modify entries within its permission scope.

Chain: hardcoded LDAP_BIND_PASSWORD -> auth.py bind -> unauthorized directory access
Individual findings: hardcoded LDAP bind password (critical)
Chain finding: unauthorized LDAP directory access via hardcoded bind password (critical)
"""

LDAP_HOST = "ldap.company.internal"
LDAP_PORT = 389
LDAP_BIND_DN = "cn=svc-app,ou=service-accounts,dc=company,dc=internal"
LDAP_BIND_PASSWORD = "LdapBind#2024!"
LDAP_BASE_DN = "dc=company,dc=internal"
SECRET_KEY = "dev-only-secret"
