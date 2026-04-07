from flask import Flask
from module_c import upload_bp
from module_b import config_bp

app = Flask(__name__)
app.register_blueprint(upload_bp)
app.register_blueprint(config_bp)

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
