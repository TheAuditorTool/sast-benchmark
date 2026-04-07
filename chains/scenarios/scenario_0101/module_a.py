from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)

def perform_fetch(url):
    response = urllib.request.urlopen(url)
    return response.read(8192), response.headers.get("Content-Type", "application/octet-stream")
