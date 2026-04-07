from urllib.parse import urlparse

ALLOWED_DOMAIN = "example.com"

def is_safe_redirect_url(url):
    if not url:
        return False
    parsed = urlparse(url)
    host = parsed.netloc.lower().split(":")[0]
    return host == ALLOWED_DOMAIN or host.endswith("." + ALLOWED_DOMAIN)
