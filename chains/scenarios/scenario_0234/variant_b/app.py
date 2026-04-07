from flask import Flask

app = Flask(__name__)

from response_builder import response_builder_bp
from dashboard import dashboard_bp

app.register_blueprint(response_builder_bp)
app.register_blueprint(dashboard_bp)

if __name__ == "__main__":
    app.run(port=5000)
