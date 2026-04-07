from flask import Flask

app = Flask(__name__)

from module_d import notify_bp

app.register_blueprint(notify_bp)

if __name__ == "__main__":
    app.run(port=5000)
