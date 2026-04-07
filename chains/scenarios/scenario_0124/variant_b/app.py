from flask import Flask

app = Flask(__name__)

from auth import oauth_bp
from routes import routes_bp

app.register_blueprint(oauth_bp)
app.register_blueprint(routes_bp)

if __name__ == "__main__":
    app.run(port=5000)
