from flask import Flask
from exporter import export_bp
from url_fetcher import fetcher_bp

app = Flask(__name__)
app.register_blueprint(export_bp)
app.register_blueprint(fetcher_bp)

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
