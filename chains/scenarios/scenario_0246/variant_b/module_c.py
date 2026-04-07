from flask import Blueprint, request, make_response
from module_b import get_cached_page, set_cached_page

views_bp = Blueprint("views", __name__)

_PAGE_TEMPLATE = """<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>{title}</title></head>
<body>
<h1>{title}</h1>
<div class="content">{body}</div>
</body>
</html>"""

@views_bp.route("/wiki/<page_name>", methods=["GET"])
def wiki_page(page_name: str):
    cache_key = request.url
    cached = get_cached_page(cache_key)
    if cached is not None:
        return make_response(cached, 200)

    theme = request.args.get("theme", "light")
    body_text = f"<p>Content of page <em>{page_name}</em> (theme: {theme})</p>"
    html = _PAGE_TEMPLATE.format(title=page_name, body=body_text)

    set_cached_page(cache_key, html)
    return make_response(html, 200)

@views_bp.route("/wiki/<page_name>/edit", methods=["POST"])
def edit_page(page_name: str):
    from flask import jsonify
    body = request.get_json(silent=True) or {}
    content = body.get("content", "")
    if not isinstance(content, str):
        return jsonify({"error": "content must be a string"}), 400

    cache_key = request.host_url.rstrip("/") + f"/wiki/{page_name}"
    html = _PAGE_TEMPLATE.format(title=page_name, body=content)
    set_cached_page(cache_key, html)
    return jsonify({"status": "saved", "page": page_name}), 200
