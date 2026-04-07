from flask import Flask
from module_b import scheduler_bp
from module_c import runner_bp

app = Flask(__name__)
app.register_blueprint(scheduler_bp)
app.register_blueprint(runner_bp)

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
