from flask import request, redirect, jsonify
from app import app
from redirect_utils import is_safe_redirect_url

@app.route("/go")
def go():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url required"}), 400

# vuln-code-snippet start ChainScenario0210A
    if not is_safe_redirect_url(url):
        return jsonify({"error": "Redirect not allowed"}), 400
    return redirect(url)  # vuln-code-snippet target-line ChainScenario0210A
# vuln-code-snippet end ChainScenario0210A

if __name__ == "__main__":
    app.run(port=5000)
