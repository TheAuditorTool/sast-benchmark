from flask import Flask

app = Flask(__name__)

from views import eval_bp

app.register_blueprint(eval_bp)

if __name__ == "__main__":
    app.run(port=5000)
