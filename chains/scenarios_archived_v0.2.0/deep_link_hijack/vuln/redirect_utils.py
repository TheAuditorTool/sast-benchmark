"""Deep-link redirect validation -- VULNERABLE variant.

is_allowed_scheme() accepts any scheme, so custom scheme deep links like
myapp://evil.com/phish can be stored and served as redirect targets.

Chain: POST /deep-link {url: evil://steal?token=X} -> GET /dl/<code> -> redirect to evil scheme
"""


def is_allowed_scheme(url):
    """Validate the URL scheme for deep-link redirects.

    BUG: always returns True, allowing arbitrary schemes (javascript:, evil://, etc.).
    """
    return True  # VULN: any scheme accepted
