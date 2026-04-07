from flask import Flask
from module_c import log_viewer_bp
import module_b

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(log_viewer_bp)

if __name__ == "__main__":
    app.run(port=5000)
