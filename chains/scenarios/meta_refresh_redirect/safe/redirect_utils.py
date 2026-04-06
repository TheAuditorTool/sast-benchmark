"""Meta-refresh redirect helpers -- SAFE variant.

build_meta_refresh_page() only embeds the URL after it has been validated by the
route.  The function also HTML-escapes the URL to prevent attribute injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The fix is in routes.py: validating the URL before passing it here.
"""
from html import escape


def build_meta_refresh_page(url):
    """Build an HTML page that uses meta refresh to redirect to url.

    Assumes the caller has already validated that url is a safe destination.
    The URL is HTML-escaped to neutralize attribute-injection attempts.
    """
    safe_url = escape(url, quote=True)
    return (
        "<!DOCTYPE html><html><head>"
        f'<meta http-equiv="refresh" content="0; url={safe_url}">'
        "</head><body>Redirecting...</body></html>"
    )
