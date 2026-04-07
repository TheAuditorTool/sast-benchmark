from flask import Flask

app = Flask(__name__)

from module_b import api_bp
from module_a import admin_bp

app.register_blueprint(api_bp)
app.register_blueprint(admin_bp)

if __name__ == "__main__":
    app.run(port=5000)
