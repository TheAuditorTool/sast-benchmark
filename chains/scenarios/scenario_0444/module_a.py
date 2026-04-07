from flask import Flask

app = Flask(__name__)
app.secret_key = "dev-secret-key"

import module_c  
