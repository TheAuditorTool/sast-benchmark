import re
from flask import Blueprint, request, jsonify
from url_fetcher import fetch_resource

export_bp = Blueprint("export", __name__)

_RENDER_URL_PATTERN = re.compile(r'(?:src|href)=["\']([^"\']+)["\']', re.IGNORECASE)

def render_html_to_pdf(html: str) -> bytes:
    fetched_sizes = {}
    for url in _RENDER_URL_PATTERN.findall(html):
        try:
            data = fetch_resource(url)
            fetched_sizes[url] = len(data)
        except Exception:
            fetched_sizes[url] = -1
    
    summary = ", ".join(f"{u}:{s}" for u, s in fetched_sizes.items())
    return f"%PDF-1.4 stub resources=[{summary}]".encode()

# vuln-code-snippet start ChainScenario0018B
@export_bp.route("/export/pdf", methods=["POST"])
def export_pdf():
    body = request.get_json(silent=True)
    if not body or "html" not in body:
        return jsonify({"error": "JSON body with 'html' field required"}), 400

    user_html = body["html"]
    if not isinstance(user_html, str) or len(user_html) > 512_000:
        return jsonify({"error": "html must be a string under 512 KB"}), 400

    pdf_bytes = render_html_to_pdf(user_html)  # vuln-code-snippet target-line ChainScenario0018B

    return jsonify({
        "status": "rendered",
        "size_bytes": len(pdf_bytes),
    }), 200
# vuln-code-snippet end ChainScenario0018B
