from flask import Flask

app = Flask(__name__)

from module_c import reports_bp
from module_b import file_server_bp

app.register_blueprint(reports_bp)
app.register_blueprint(file_server_bp)

if __name__ == "__main__":
    app.run(port=5000)
