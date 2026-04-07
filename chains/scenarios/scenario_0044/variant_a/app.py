from flask import Flask
from serve import serve_bp
import storage

app = Flask(__name__)
app.secret_key = "dev-secret-key-do-not-use-in-prod"
app.register_blueprint(serve_bp)

if __name__ == "__main__":
    app.run(port=5000)
