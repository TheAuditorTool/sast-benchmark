require_relative 'shared'
require 'securerandom'

def oauth_callback_safe(req)
  code = req.param('code')
  state = req.param('state')
  session_state = req.cookie('oauth_state')
  return BenchmarkResponse.error('CSRF', 403) unless state == session_state
  BenchmarkResponse.ok("oauth code: #{code}")
end
