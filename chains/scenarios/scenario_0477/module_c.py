from urllib.parse import urlparse
from flask import request, Response
from module_a import app
from module_b import build_redirect_page

ALLOWED_HOST = "app.example.com"

def _safe_fallback(url):
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return "/dashboard"
    return url

@app.route("/redirect")
def js_redirect():
    fallback = _safe_fallback(request.args.get("fallback", "/dashboard"))

    html = build_redirect_page(fallback)
    return Response(html, mimetype="text/html")

if __name__ == "__main__":
    app.run(port=5000)
