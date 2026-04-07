"""Redirect handler -- IDENTICAL between vuln/ and safe/.

Simulates an open redirect on a trusted host. The fetcher's allowlist
permits requests to trusted-cdn.example.com, but that host redirects to
an attacker-controlled URL via a 302 response.

Chain: attacker -> /fetch?url=https://trusted-cdn.example.com/r?to=http://169.254.169.254/...
       fetcher follows the redirect without re-checking the destination host

Redirect targets served by trusted-cdn.example.com/r?to=<url>:
  Any URL supplied as the 'to' query parameter -- open redirect, no validation.
  This represents a third-party CDN with an open redirect, or a compromised
  but allowlisted host.
"""

REDIRECT_HOST = "trusted-cdn.example.com"
REDIRECT_PATH = "/r"
# The redirect endpoint issues a 302 to the value of the 'to' query parameter
# without validating that the destination is safe.
