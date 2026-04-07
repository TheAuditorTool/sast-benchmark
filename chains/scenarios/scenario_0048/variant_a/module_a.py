from flask import Flask

app = Flask(__name__)

from module_d import render_bp

app.register_blueprint(render_bp)

if __name__ == "__main__":
    app.run(port=5000)
