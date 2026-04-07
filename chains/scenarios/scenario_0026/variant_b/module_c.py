from flask import request, redirect
from module_a import app
from module_b import build_redirect_url

@app.route("/go")
def go():
    path = request.args.get("to", "/")
    if not path.startswith("/"):
        path = "/"

    url = build_redirect_url(path)
    return redirect(url)

if __name__ == "__main__":
    app.run(port=5000)
