"""Content caching service -- IDENTICAL between vuln/ and safe/ variants.

Flask application for a community wiki where pages are cached in memory for
performance. The cache layer and the page rendering layer are separate modules,
allowing the cache key construction bug in cache.py to be tested independently
from the XSS rendering sink in views.py.

Chain: incomplete cache key -> attacker poisons cache with XSS payload ->
       subsequent victims receive poisoned cached response containing script
Individual findings: none in this file
Chain finding: cache poisoning enabling stored XSS delivery (critical, CWE-79)
"""
from flask import Flask
from cache import cache_bp
from views import views_bp

app = Flask(__name__)
app.register_blueprint(cache_bp)
app.register_blueprint(views_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)
