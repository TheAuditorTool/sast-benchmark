from flask import Flask, request, jsonify
from module_b import ensure_upload_dir
from module_c import save_upload

app = Flask(__name__)

MAX_SIZE = 1 * 1024 * 1024  

@app.route("/upload", methods=["POST"])
def upload():
    filename = request.args.get("filename")
    if not filename:
        return jsonify({"error": "filename query parameter is required"}), 400

    data = request.get_data()
    if len(data) > MAX_SIZE:
        return jsonify({"error": "File too large"}), 413

    result, status = save_upload(filename, data)
    return jsonify(result), status

@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok"}), 200

if __name__ == "__main__":
    ensure_upload_dir()
    app.run(port=5000)
