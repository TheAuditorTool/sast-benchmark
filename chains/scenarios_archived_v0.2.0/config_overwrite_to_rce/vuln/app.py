"""Dynamic plugin configuration service -- IDENTICAL between vuln/ and safe/ variants.

Flask application that allows administrators to upload new plugin configuration
files. On startup and after each upload, the application dynamically loads the
active configuration using exec(). If an attacker can upload a file into the
config directory, they can inject arbitrary Python code that will be executed.

Chain: unrestricted upload writes to config dir -> config_loader exec()'s file -> RCE
Individual findings: none in this file
Chain finding: config overwrite via upload + exec = RCE (critical, CWE-94)
"""
from flask import Flask
from upload import upload_bp
from config_loader import config_bp

app = Flask(__name__)
app.register_blueprint(upload_bp)
app.register_blueprint(config_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)
