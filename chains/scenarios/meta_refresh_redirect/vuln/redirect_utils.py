"""Meta-refresh redirect helpers -- VULNERABLE variant.

build_meta_refresh_page() embeds the user-supplied URL directly into a
<meta http-equiv="refresh"> tag without sanitization.  This is an open
redirect via HTML rather than the Location header, which may bypass some filters.

Chain: GET /loading?next=https://evil.com -> meta refresh to phishing page
"""


def build_meta_refresh_page(url):
    """Build an HTML page that uses meta refresh to redirect to url.

    BUG: url is embedded without host validation or HTML escaping.
    """
    return (
        "<!DOCTYPE html><html><head>"
        f'<meta http-equiv="refresh" content="0; url={url}">'
        "</head><body>Redirecting...</body></html>"
    )
