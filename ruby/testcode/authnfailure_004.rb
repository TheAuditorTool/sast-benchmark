require_relative 'shared'

module Devise
  def self.confirmable?; true; end
  def self.lockable?; true; end
end

# vuln-code-snippet start ruby_authn_devise_full
def login_devise(req)
  username = req.param('username')
  password = req.param('password')
  return BenchmarkResponse.error('unconfirmed', 403) unless Devise.confirmable?
  return BenchmarkResponse.error('locked', 423) unless Devise.lockable? # vuln-code-snippet safe-line ruby_authn_devise_full
  BenchmarkResponse.ok("authenticated: #{username}")
end
# vuln-code-snippet end ruby_authn_devise_full
