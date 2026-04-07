from flask import Blueprint, make_response
from module_b import handle_conditional, RESOURCE_BODY

views_bp = Blueprint("views", __name__)

@views_bp.route("/resource")
def resource():
    resp = make_response(RESOURCE_BODY)
    resp.headers["Cache-Control"] = "public, max-age=300"
    return handle_conditional(resp)
