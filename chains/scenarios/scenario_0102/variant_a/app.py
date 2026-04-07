from flask import Flask
from static_handler import static_handler_bp
import config

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(static_handler_bp)

if __name__ == "__main__":
    app.run(port=5000)
