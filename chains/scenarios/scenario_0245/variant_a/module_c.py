from flask import request, Response
from module_a import app
from module_b import build_meta_refresh_page

@app.route("/loading")
def loading():
    next_url = request.args.get("next", "/dashboard")

    html = build_meta_refresh_page(next_url)
    return Response(html, mimetype="text/html")

if __name__ == "__main__":
    app.run(port=5000)
