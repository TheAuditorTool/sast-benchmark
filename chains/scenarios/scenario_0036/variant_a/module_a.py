from flask import Flask
from module_b import backups_bp
import module_c

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(backups_bp)

if __name__ == "__main__":
    app.run(port=5000)
