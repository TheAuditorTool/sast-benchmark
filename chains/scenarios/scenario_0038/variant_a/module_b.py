def build_redirect_page(fallback_url):
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
