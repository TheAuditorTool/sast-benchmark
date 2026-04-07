from flask import request, redirect, session
from app import app
from redirect_utils import is_safe_redirect_url

@app.route("/logout")
def logout():
    session.clear()
    redirect_url = request.args.get("redirect", "/")

# vuln-code-snippet start ChainScenario0116A
    if not is_safe_redirect_url(redirect_url):
        redirect_url = "/"
    return redirect(redirect_url)  # vuln-code-snippet target-line ChainScenario0116A
# vuln-code-snippet end ChainScenario0116A

if __name__ == "__main__":
    app.run(port=5000)
