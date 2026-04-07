from flask import Flask

app = Flask(__name__)

from module_c import parser_bp
from module_b import internal_bp

app.register_blueprint(parser_bp)
app.register_blueprint(internal_bp)

if __name__ == "__main__":
    app.run(port=5000)
