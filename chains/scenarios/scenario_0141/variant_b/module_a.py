from flask import Flask

app = Flask(__name__)

from module_b import directory_bp
from module_c import user_service_bp

app.register_blueprint(directory_bp)
app.register_blueprint(user_service_bp)

if __name__ == "__main__":
    app.run(port=5000)
