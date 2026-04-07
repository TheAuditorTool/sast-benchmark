from flask import request, redirect
from app import app
from redirect_utils import build_redirect_url

@app.route("/go")
def go():
    path = request.args.get("to", "/")
    if not path.startswith("/"):
        path = "/"

# vuln-code-snippet start ChainScenario0026B
    url = build_redirect_url(path)
    return redirect(url)  # vuln-code-snippet target-line ChainScenario0026B
# vuln-code-snippet end ChainScenario0026B

if __name__ == "__main__":
    app.run(port=5000)
