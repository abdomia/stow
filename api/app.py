from flask import Flask, jsonify

app = Flask(__name__)
print(__name__)


@app.route("/")
def python_route():
    return jsonify(message="Hello from Flask!")
