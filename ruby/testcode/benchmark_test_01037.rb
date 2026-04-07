require_relative 'shared'

module Devise
  def self.confirmable?; true; end
  def self.lockable?; true; end
end

def login_devise(req)
  username = req.param('username')
  password = req.param('password')
  return BenchmarkResponse.error('unconfirmed', 403) unless Devise.confirmable?
  return BenchmarkResponse.error('locked', 423) unless Devise.lockable?
  BenchmarkResponse.ok("authenticated: #{username}")
end
