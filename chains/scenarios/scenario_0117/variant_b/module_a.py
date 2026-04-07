from flask import Flask
from module_b import files_bp
from module_c import git_bp

app = Flask(__name__)
app.register_blueprint(files_bp)
app.register_blueprint(git_bp)

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
