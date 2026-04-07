from flask import Flask
from viewer import viewer_bp
import config

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(viewer_bp)

if __name__ == "__main__":
    app.run(port=5000)
