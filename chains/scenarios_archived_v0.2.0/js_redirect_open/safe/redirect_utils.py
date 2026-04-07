"""JS fragment redirect helpers -- SAFE variant.

build_redirect_page() uses a hardcoded server-validated fallback URL.
The JavaScript still reads the hash, but the hash destination is validated
client-side against a hardcoded allowed host list, and the fallback is
always a safe internal path.

This file is IDENTICAL between vuln/ and safe/ variants.
The fix is in routes.py: validating ?fallback= before embedding it.
"""


def build_redirect_page(fallback_url):
    """Build an HTML page that reads the hash for redirect or falls back to fallback_url.

    The JS also validates that the hash destination starts with the allowed origin.
    """
    return f"""<!DOCTYPE html>
<html>
<head><title>Redirecting...</title></head>
<body>
<script>
var allowedOrigin = window.location.origin;
var dest = window.location.hash.slice(1);
try {{
  var u = new URL(dest, window.location.origin);
  if (u.origin !== allowedOrigin) {{ dest = "{fallback_url}"; }}
}} catch(e) {{
  dest = "{fallback_url}";
}}
if (!dest) {{ dest = "{fallback_url}"; }}
window.location.replace(dest);
</script>
<noscript>
  <meta http-equiv="refresh" content="0; url={fallback_url}">
</noscript>
</body>
</html>"""
