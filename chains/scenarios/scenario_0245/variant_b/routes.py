from urllib.parse import urlparse
from flask import request, Response
from app import app
from redirect_utils import build_meta_refresh_page

ALLOWED_HOST = "app.example.com"

def _safe_next(url):
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return "/dashboard"
    return url

@app.route("/loading")
def loading():
    next_url = _safe_next(request.args.get("next", "/dashboard"))

# vuln-code-snippet start ChainScenario0245B
    html = build_meta_refresh_page(next_url)
    return Response(html, mimetype="text/html")  # vuln-code-snippet target-line ChainScenario0245B
# vuln-code-snippet end ChainScenario0245B

if __name__ == "__main__":
    app.run(port=5000)
