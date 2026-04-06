"""JS fragment redirect helpers -- VULNERABLE variant.

build_redirect_page() embeds a JavaScript redirect that reads window.location.hash
without any server-side validation.  The hash is not sent to the server, so
scanner tools miss this, but a client-side attacker can control it via a crafted URL.

This file is IDENTICAL between vuln/ and safe/ variants.
The vulnerability is in routes.py: passing the ?fallback= param into the JS unescaped.
"""


def build_redirect_page(fallback_url):
    """Build an HTML page that reads the hash for redirect or falls back to fallback_url."""
    return f"""<!DOCTYPE html>
<html>
<head><title>Redirecting...</title></head>
<body>
<script>
var dest = window.location.hash.slice(1);
if (!dest) {{ dest = "{fallback_url}"; }}
window.location.replace(dest);
</script>
<noscript>
  <meta http-equiv="refresh" content="0; url={fallback_url}">
</noscript>
</body>
</html>"""
