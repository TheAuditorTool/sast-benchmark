def build_meta_refresh_page(url):
    return (
        "<!DOCTYPE html><html><head>"
        f'<meta http-equiv="refresh" content="0; url={url}">'
        "</head><body>Redirecting...</body></html>"
    )
