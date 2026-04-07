from flask import request, jsonify
from models import app, ARTICLES
from auth import require_viewer, require_editor

@app.route("/articles/<article_id>", methods=["GET"])
@require_viewer
def get_article(article_id):
    article = ARTICLES.get(article_id)
    if article is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(article)

# vuln-code-snippet start ChainScenario0186B
@app.route("/articles/<article_id>", methods=["PATCH"])
@require_editor
def update_article(article_id):
    article = ARTICLES.get(article_id)
    if article is None:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    for field in ("title", "body", "published"):
        if field in data:
            article[field] = data[field]  # vuln-code-snippet target-line ChainScenario0186B
    return jsonify({"status": "updated", "article": article})
# vuln-code-snippet end ChainScenario0186B

if __name__ == "__main__":
    app.run(port=5000)
