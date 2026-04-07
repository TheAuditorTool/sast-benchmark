from flask import request, redirect, jsonify
from module_a import app
from module_b import is_safe_redirect_url

@app.route("/go")
def go():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url required"}), 400

    if not is_safe_redirect_url(url):
        return jsonify({"error": "Redirect not allowed"}), 400
    return redirect(url)

if __name__ == "__main__":
    app.run(port=5000)
