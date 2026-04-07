from flask import Flask

app = Flask(__name__)

from module_b import loader_bp

app.register_blueprint(loader_bp)

if __name__ == "__main__":
    app.run(port=5000)
