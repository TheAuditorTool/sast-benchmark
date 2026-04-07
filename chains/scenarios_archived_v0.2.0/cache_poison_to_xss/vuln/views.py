"""Page rendering views -- IDENTICAL between vuln/ and safe/ variants.

Renders wiki page content retrieved from the cache. The content stored in
the cache may contain HTML that originated from user input (the page body
is stored as-is). This module is the XSS sink: it passes cached content
into a raw HTML response. It is safe in isolation when the cache contains
legitimate content, but becomes the delivery vehicle for XSS once the
cache is poisoned via the bug in cache.py.

Chain: poisoned cache entry (from cache.py) -> get_cached_page() returns
       attacker payload -> render_page() embeds it unescaped in HTML response
       -> XSS executes in victim's browser
Individual findings: unescaped content in HTML response (medium, context-dependent)
Chain finding: when combined with cache poisoning, delivers XSS to all cache
               consumers (critical, CWE-79)
"""
from flask import Blueprint, request, make_response
from cache import get_cached_page, set_cached_page

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
    """Render a wiki page, serving from cache when available.

    Query parameters (e.g. ?theme=dark) affect the rendered output and
    should be part of the cache key -- but cache.py's build_cache_key()
    omits them in the vulnerable variant.
    """
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
    """Accept a new page body and store it under the page's cache key.

    The submitted body is stored directly in the cache without sanitization.
    This is the injection point: an attacker submits a script tag as the body.
    """
    from flask import jsonify
    body = request.get_json(silent=True) or {}
    content = body.get("content", "")
    if not isinstance(content, str):
        return jsonify({"error": "content must be a string"}), 400

    cache_key = request.host_url.rstrip("/") + f"/wiki/{page_name}"
    html = _PAGE_TEMPLATE.format(title=page_name, body=content)
    set_cached_page(cache_key, html)
    return jsonify({"status": "saved", "page": page_name}), 200
