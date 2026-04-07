from flask import request, jsonify
from module_b import app, ARTICLES
from module_a import require_viewer

@app.route("/articles/<article_id>", methods=["GET"])
@require_viewer
def get_article(article_id):
    article = ARTICLES.get(article_id)
    if article is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(article)

@app.route("/articles/<article_id>", methods=["PATCH"])
@require_viewer
def update_article(article_id):
    article = ARTICLES.get(article_id)
    if article is None:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    for field in ("title", "body", "published"):
        if field in data:
            article[field] = data[field]
    return jsonify({"status": "updated", "article": article})

if __name__ == "__main__":
    app.run(port=5000)
