from urllib.parse import urlparse
from flask import request, Response
from module_a import app
from module_b import build_meta_refresh_page

ALLOWED_HOST = "app.example.com"

def _safe_next(url):
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return "/dashboard"
    return url

@app.route("/loading")
def loading():
    next_url = _safe_next(request.args.get("next", "/dashboard"))

    html = build_meta_refresh_page(next_url)
    return Response(html, mimetype="text/html")

if __name__ == "__main__":
    app.run(port=5000)
