"""User file storage service -- IDENTICAL between vuln/ and safe/ variants.

Flask application providing per-user file storage. Files are written to a
per-user upload directory. A separate read endpoint serves file contents by
name. If a stored file is a symlink created by an attacker (or via a TOCTOU
race), and the reader follows symlinks, the attacker can read arbitrary files
on the server filesystem.

Chain: attacker creates symlink in storage -> reader follows symlink -> sensitive file read
Individual findings: none in this file
Chain finding: symlink following enables arbitrary file read (high, CWE-22)
"""
from flask import Flask
from storage import storage_bp
from reader import reader_bp

app = Flask(__name__)
app.register_blueprint(storage_bp)
app.register_blueprint(reader_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)
