from flask import Flask

app = Flask(__name__)

from module_b import content_api_bp
from module_c import page_renderer_bp

app.register_blueprint(content_api_bp)
app.register_blueprint(page_renderer_bp)

if __name__ == "__main__":
    app.run(port=5000)
