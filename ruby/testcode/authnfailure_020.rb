require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_authn_oauth_state
def oauth_callback_safe(req)
  code = req.param('code')
  state = req.param('state')
  session_state = req.cookie('oauth_state')
  return BenchmarkResponse.error('CSRF', 403) unless state == session_state # vuln-code-snippet safe-line ruby_authn_oauth_state
  BenchmarkResponse.ok("oauth code: #{code}")
end
# vuln-code-snippet end ruby_authn_oauth_state
