from html import escape

def build_meta_refresh_page(url):
    safe_url = escape(url, quote=True)
    return (
        "<!DOCTYPE html><html><head>"
        f'<meta http-equiv="refresh" content="0; url={safe_url}">'
        "</head><body>Redirecting...</body></html>"
    )
