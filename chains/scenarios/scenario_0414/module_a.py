from flask import Flask

app = Flask(__name__)

from module_c import response_builder_bp
from module_b import dashboard_bp

app.register_blueprint(response_builder_bp)
app.register_blueprint(dashboard_bp)

if __name__ == "__main__":
    app.run(port=5000)
