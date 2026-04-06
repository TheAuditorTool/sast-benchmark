require_relative 'shared'

OAUTH_CLIENT_SECRET = 'dGhpcyBpcyBhIHNlY3JldCBrZXk='

# vuln-code-snippet start ruby_hardcoded_oauth
def oauth_callback(req)
  config = { client_id: 'app_id', client_secret: OAUTH_CLIENT_SECRET } # vuln-code-snippet vuln-line ruby_hardcoded_oauth
  BenchmarkResponse.ok("oauth configured")
end
# vuln-code-snippet end ruby_hardcoded_oauth
