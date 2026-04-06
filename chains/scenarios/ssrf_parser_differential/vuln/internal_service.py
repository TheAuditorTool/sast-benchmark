"""Internal API service configuration -- IDENTICAL between vuln/ and safe/.

Represents an internal admin API that trusts the network boundary and
requires no authentication. The service is reachable only from within the
VPC, so engineers considered it safe. The parser differential in proxy.py
allows an attacker to construct a URL that passes the manual string check
but is fetched by urllib as a request to this internal host.

Chain: attacker -> /proxy?url=http://trusted.example.com@10.0.2.5:8080/admin -> internal API

Internal admin endpoints:
  http://10.0.2.5:8080/admin/users       -> Full user list with password hashes
  http://10.0.2.5:8080/admin/config      -> Runtime configuration (DB DSN, API keys)
  http://10.0.2.5:8080/admin/reset-admin -> Resets the admin password (GET, no CSRF)
"""

INTERNAL_API_HOST = "10.0.2.5"
INTERNAL_API_PORT = 8080
INTERNAL_API_BASE = "http://{}:{}/admin".format(INTERNAL_API_HOST, INTERNAL_API_PORT)
INTERNAL_API_TOKEN = None
