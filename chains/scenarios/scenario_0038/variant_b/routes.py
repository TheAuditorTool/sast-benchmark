from flask import request, Response
from app import app
from redirect_utils import build_redirect_page

@app.route("/redirect")
def js_redirect():
    fallback = request.args.get("fallback", "/dashboard")

# vuln-code-snippet start ChainScenario0038B
    html = build_redirect_page(fallback)
    return Response(html, mimetype="text/html")  # vuln-code-snippet target-line ChainScenario0038B
# vuln-code-snippet end ChainScenario0038B

if __name__ == "__main__":
    app.run(port=5000)
