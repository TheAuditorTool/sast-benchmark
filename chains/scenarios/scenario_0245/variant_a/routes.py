from flask import request, Response
from app import app
from redirect_utils import build_meta_refresh_page

@app.route("/loading")
def loading():
    next_url = request.args.get("next", "/dashboard")

# vuln-code-snippet start ChainScenario0245A
    html = build_meta_refresh_page(next_url)
    return Response(html, mimetype="text/html")  # vuln-code-snippet target-line ChainScenario0245A
# vuln-code-snippet end ChainScenario0245A

if __name__ == "__main__":
    app.run(port=5000)
