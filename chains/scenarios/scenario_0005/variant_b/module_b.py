from flask import Flask, request, jsonify
import urllib.request

app = Flask(__name__)

def receive_webhook():
    data = request.get_json()
    callback_url = data.get("callback_url", "")
    
    response = urllib.request.urlopen(callback_url)
    return jsonify({"status": "received", "verified": response.status == 200})

app.add_url_rule("/api/webhook", view_func=receive_webhook, methods=["POST"])
