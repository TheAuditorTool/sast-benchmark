from flask import Flask
from module_c import storage_bp
from module_b import reader_bp

app = Flask(__name__)
app.register_blueprint(storage_bp)
app.register_blueprint(reader_bp)

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
