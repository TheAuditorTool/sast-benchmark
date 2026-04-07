from flask import Flask
from module_c import i18n_bp
import module_b

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(i18n_bp)

if __name__ == "__main__":
    app.run(port=5000)
