"""Profile template rendering -- SAFE variant.

Renders stored profile data into HTML with html.escape() applied
to all user-controlled values. Even if an XSS payload is stored in
the database (via the SQLi in app.py), it will be rendered as
harmless escaped text in the browser.

Chain: stored XSS payload in DB -> escaped before rendering -> no XSS
"""
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
    """Render a user profile as an HTML page."""
    html_content = PROFILE_TEMPLATE.format(
        name=html.escape(profile_data["name"]),
        bio=html.escape(profile_data["bio"]),
        id=profile_data["id"],
    )
    response = make_response(html_content)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response
