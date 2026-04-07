require_relative 'shared'

module Warden
  def self.authenticate!(scope: :user); true; end
end

def login_warden(req)
  Warden.authenticate!(scope: :user)
  session_id = SecureRandom.hex(32)
  BenchmarkResponse.ok("session: #{session_id}")
end
