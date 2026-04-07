from flask import Flask
from profiles import profiles_bp
import file_utils

app = Flask(__name__)
app.secret_key = "dev-secret-key-do-not-use-in-prod"
app.register_blueprint(profiles_bp)

if __name__ == "__main__":
    app.run(port=5000)
