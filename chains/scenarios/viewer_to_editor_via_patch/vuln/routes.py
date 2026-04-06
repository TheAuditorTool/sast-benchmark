"""CMS article routes -- VULNERABLE variant.

GET /articles/<id> is correctly restricted to viewers.
PATCH /articles/<id> is also only gated by require_viewer, not require_editor,
so a viewer can modify article content.

Chain: viewer -> PATCH with only viewer check -> article body updated
"""
from flask import request, jsonify
from models import app, ARTICLES
from auth import require_viewer


@app.route("/articles/<article_id>", methods=["GET"])
@require_viewer
def get_article(article_id):
    """Retrieve a single article. Viewer access sufficient."""
    article = ARTICLES.get(article_id)
    if article is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(article)


# vuln-code-snippet start chain_viewer_editor_vuln
@app.route("/articles/<article_id>", methods=["PATCH"])
@require_viewer
def update_article(article_id):
    """Update article fields. VULNERABLE: only viewer check applied."""
    article = ARTICLES.get(article_id)
    if article is None:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    for field in ("title", "body", "published"):
        if field in data:
            article[field] = data[field]  # vuln-code-snippet vuln-line chain_viewer_editor_vuln
    return jsonify({"status": "updated", "article": article})
# vuln-code-snippet end chain_viewer_editor_vuln


if __name__ == "__main__":
    app.run(port=5000)
