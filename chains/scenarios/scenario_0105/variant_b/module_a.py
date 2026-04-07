from flask import Flask
from module_b import renderer_bp
import module_c as tmpl_config

app = Flask(__name__)
app.secret_key = tmpl_config.SECRET_KEY
app.register_blueprint(renderer_bp)

if __name__ == "__main__":
    app.run(port=5000)
