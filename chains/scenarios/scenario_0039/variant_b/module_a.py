from flask import Flask

app = Flask(__name__)

from module_b import response_builder_bp
from module_c import views_bp

app.register_blueprint(response_builder_bp)
app.register_blueprint(views_bp)

if __name__ == "__main__":
    app.run(port=5000)
