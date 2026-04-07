from flask import Flask

app = Flask(__name__)

from module_d import email_bp

app.register_blueprint(email_bp)

if __name__ == "__main__":
    app.run(port=5000)
