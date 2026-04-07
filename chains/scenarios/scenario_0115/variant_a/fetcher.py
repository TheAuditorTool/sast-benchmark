from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse, urljoin

app = Flask(__name__)

ALLOWED_HOSTS = {"trusted-cdn.example.com", "assets.example.com", "images.example.com"}
MAX_REDIRECTS = 5

class _NoRedirectHandler(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None

_no_redirect_opener = urllib.request.build_opener(_NoRedirectHandler())

def _check_host(url):
    parsed = urlparse(url)
    return parsed.hostname in ALLOWED_HOSTS and parsed.scheme in ("http", "https")

# vuln-code-snippet start ChainScenario0115A
def fetch_resource():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    if not _check_host(url):
        return jsonify({"error": "URL host not in allowlist"}), 403
    hops = 0
    current_url = url
    while hops < MAX_REDIRECTS:
        try:
            response = _no_redirect_opener.open(current_url)  # vuln-code-snippet target-line ChainScenario0115A
            body = response.read(8192)
            content_type = response.headers.get("Content-Type", "application/octet-stream")
            return body, 200, {"Content-Type": content_type}
        except urllib.error.HTTPError as exc:
            if exc.code in (301, 302, 303, 307, 308):
                location = exc.headers.get("Location", "")
                next_url = urljoin(current_url, location)
                if not _check_host(next_url):
                    return jsonify({"error": "Redirect destination not in allowlist"}), 403
                current_url = next_url
                hops += 1
            else:
                return jsonify({"error": "HTTP {}".format(exc.code)}), 502
        except urllib.error.URLError as exc:
            return jsonify({"error": str(exc)}), 502
    return jsonify({"error": "Too many redirects"}), 502
# vuln-code-snippet end ChainScenario0115A

app.add_url_rule("/api/fetch", view_func=fetch_resource)

if __name__ == "__main__":
    app.run(port=5015)
