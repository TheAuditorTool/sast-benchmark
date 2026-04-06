"""CMS article routes -- SAFE variant.

PATCH /articles/<id> is now gated by require_editor, ensuring only users
with editor role or above can modify articles.

Chain: viewer -> PATCH requires editor role -> 403 (chain broken)
"""
from flask import request, jsonify
from models import app, ARTICLES
from auth import require_viewer, require_editor


@app.route("/articles/<article_id>", methods=["GET"])
@require_viewer
def get_article(article_id):
    """Retrieve a single article. Viewer access sufficient."""
    article = ARTICLES.get(article_id)
    if article is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(article)


# vuln-code-snippet start chain_viewer_editor_safe
@app.route("/articles/<article_id>", methods=["PATCH"])
@require_editor
def update_article(article_id):
    """Update article fields. SAFE: editor role enforced."""
    article = ARTICLES.get(article_id)
    if article is None:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    for field in ("title", "body", "published"):
        if field in data:
            article[field] = data[field]  # vuln-code-snippet safe-line chain_viewer_editor_safe
    return jsonify({"status": "updated", "article": article})
# vuln-code-snippet end chain_viewer_editor_safe


if __name__ == "__main__":
    app.run(port=5000)
