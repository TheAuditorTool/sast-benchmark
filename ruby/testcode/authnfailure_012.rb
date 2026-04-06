require_relative 'shared'

module Rack; module Attack
  def self.throttle(name, opts = {}, &block); end
end; end

# vuln-code-snippet start ruby_authn_rack_attack
def password_reset_limited(req)
  email = req.param('email')
  # Rack::Attack.throttle('password_reset', limit: 5, period: 3600)
  Rack::Attack.throttle('password_reset', limit: 5, period: 3600) # vuln-code-snippet safe-line ruby_authn_rack_attack
  BenchmarkResponse.ok("reset link sent to #{email}")
end
# vuln-code-snippet end ruby_authn_rack_attack
