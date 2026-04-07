from flask import request, Response
from module_a import app
from module_b import build_redirect_page

@app.route("/redirect")
def js_redirect():
    fallback = request.args.get("fallback", "/dashboard")

    html = build_redirect_page(fallback)
    return Response(html, mimetype="text/html")

if __name__ == "__main__":
    app.run(port=5000)
