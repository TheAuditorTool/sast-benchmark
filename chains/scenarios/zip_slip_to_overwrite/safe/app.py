"""Plugin distribution service -- IDENTICAL between vuln/ and safe/ variants.

Flask application for a plugin marketplace. Users upload plugin bundles as
zip archives. The server extracts them and uses a config.json inside the
archive to register the plugin. If extraction allows path traversal (zip slip),
an attacker can overwrite application config files which are loaded at startup.

Chain: zip slip in extractor -> overwrites app config -> config_loader eval -> RCE
Individual findings: none in this file
Chain finding: zip slip + config overwrite enables code execution (critical, CWE-434)
"""
from flask import Flask
from extractor import extractor_bp
from config_loader import config_bp

app = Flask(__name__)
app.register_blueprint(extractor_bp)
app.register_blueprint(config_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)
