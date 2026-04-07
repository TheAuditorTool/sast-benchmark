from flask import Flask

app = Flask(__name__)

from views import greet_bp

app.register_blueprint(greet_bp)

if __name__ == "__main__":
    app.run(port=5000)
