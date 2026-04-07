import re

SUBDOMAIN_PATTERN = re.compile(r"https?://.*\.example\.com(/.*)?$")

def is_safe_redirect_url(url):
    return bool(SUBDOMAIN_PATTERN.match(url))  
