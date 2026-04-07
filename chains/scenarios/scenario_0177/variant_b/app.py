from flask import Flask
from extractor import extractor_bp
from config_loader import config_bp

app = Flask(__name__)
app.register_blueprint(extractor_bp)
app.register_blueprint(config_bp)

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
