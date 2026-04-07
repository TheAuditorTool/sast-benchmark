from flask import Flask
from module_c import profiles_bp
import module_b

app = Flask(__name__)
app.secret_key = "dev-secret-key-do-not-use-in-prod"
app.register_blueprint(profiles_bp)

if __name__ == "__main__":
    app.run(port=5000)
