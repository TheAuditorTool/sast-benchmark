from flask import Flask

app = Flask(__name__)

from module_c import error_handler_bp
from module_a import admin_actions_bp

app.register_blueprint(error_handler_bp)
app.register_blueprint(admin_actions_bp)

if __name__ == "__main__":
    app.run(port=5000)
