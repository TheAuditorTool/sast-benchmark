from flask import Flask
from module_c import pm_bp
from module_b import env_bp

app = Flask(__name__)
app.register_blueprint(pm_bp)
app.register_blueprint(env_bp)

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
