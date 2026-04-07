from flask import Flask

app = Flask(__name__)

from module_b import task_api_bp
from module_c import worker_bp

app.register_blueprint(task_api_bp)
app.register_blueprint(worker_bp)

if __name__ == "__main__":
    app.run(port=5000)
