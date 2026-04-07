from flask import Flask

app = Flask(__name__)

from module_d import report_bp

app.register_blueprint(report_bp)

if __name__ == "__main__":
    app.run(port=5000)
