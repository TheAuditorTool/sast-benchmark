require_relative 'shared'

OAUTH_CLIENT_SECRET = 'dGhpcyBpcyBhIHNlY3JldCBrZXk='

def oauth_callback(req)
  config = { client_id: 'app_id', client_secret: OAUTH_CLIENT_SECRET }
  BenchmarkResponse.ok("oauth configured")
end
