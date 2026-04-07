import html
from flask import make_response

PROFILE_TEMPLATE = """<!DOCTYPE html>
<html>
<head><title>Profile - {name}</title></head>
<body>
  <div class="profile-card">
    <h1>{name}</h1>
    <p class="bio">{bio}</p>
    <span class="profile-id">#{id}</span>
  </div>
</body>
</html>"""

def render_profile(profile_data):
    html_content = PROFILE_TEMPLATE.format(
        name=html.escape(profile_data["name"]),
        bio=html.escape(profile_data["bio"]),
        id=profile_data["id"],
    )
    response = make_response(html_content)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response
