from flask import Flask

app = Flask(__name__)

from module_b import calculator_bp
from module_c import data_store_bp

app.register_blueprint(calculator_bp)
app.register_blueprint(data_store_bp)

if __name__ == "__main__":
    app.run(port=5000)
