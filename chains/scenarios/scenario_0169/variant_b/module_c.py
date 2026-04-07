import os

LDAP_HOST = "ldap.company.internal"
LDAP_PORT = 389
LDAP_BIND_DN = "cn=svc-app,ou=service-accounts,dc=company,dc=internal"
LDAP_BIND_PASSWORD = os.environ.get("LDAP_BIND_PASSWORD", "")
LDAP_BASE_DN = "dc=company,dc=internal"
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
