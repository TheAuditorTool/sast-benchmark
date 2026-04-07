require_relative 'shared'
require 'securerandom'

def set_cross_site_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('cross-site cookie set')
  response.headers['Set-Cookie'] = "cross_site=#{token}; SameSite=None"
  response
end
