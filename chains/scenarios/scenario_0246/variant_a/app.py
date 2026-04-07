from flask import Flask
from cache import cache_bp
from views import views_bp

app = Flask(__name__)
app.register_blueprint(cache_bp)
app.register_blueprint(views_bp)

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
