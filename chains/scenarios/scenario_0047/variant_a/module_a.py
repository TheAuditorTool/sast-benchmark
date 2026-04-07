from flask import Flask
from module_b import importer_bp
from module_c import processor_bp

app = Flask(__name__)
app.register_blueprint(importer_bp)
app.register_blueprint(processor_bp)

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
