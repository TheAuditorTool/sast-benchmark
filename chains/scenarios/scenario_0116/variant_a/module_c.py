from flask import request, redirect, session
from module_a import app
from module_b import is_safe_redirect_url

@app.route("/logout")
def logout():
    session.clear()
    redirect_url = request.args.get("redirect", "/")

    if not is_safe_redirect_url(redirect_url):
        redirect_url = "/"
    return redirect(redirect_url)

if __name__ == "__main__":
    app.run(port=5000)
