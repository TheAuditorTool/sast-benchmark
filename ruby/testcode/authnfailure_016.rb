require_relative 'shared'

module Warden
  def self.authenticate!(scope: :user); true; end
end

# vuln-code-snippet start ruby_authn_warden_session
def login_warden(req)
  Warden.authenticate!(scope: :user)
  session_id = SecureRandom.hex(32) # vuln-code-snippet safe-line ruby_authn_warden_session
  BenchmarkResponse.ok("session: #{session_id}")
end
# vuln-code-snippet end ruby_authn_warden_session
