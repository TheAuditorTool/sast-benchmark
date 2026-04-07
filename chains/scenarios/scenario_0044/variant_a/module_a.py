from flask import Flask
from module_b import serve_bp
import module_c

app = Flask(__name__)
app.secret_key = "dev-secret-key-do-not-use-in-prod"
app.register_blueprint(serve_bp)

if __name__ == "__main__":
    app.run(port=5000)
