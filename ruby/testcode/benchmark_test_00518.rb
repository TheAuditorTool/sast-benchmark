require_relative 'shared'

module Rack; module Attack
  def self.throttle(name, opts = {}, &block); end
end; end

def password_reset_limited(req)
  email = req.param('email')
  Rack::Attack.throttle('password_reset', limit: 5, period: 3600)
  BenchmarkResponse.ok("reset link sent to #{email}")
end
