require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.bind_as(opts = {}); true; end
end; end

def auth_bind(req)
  username = req.param('username')
  password = req.param('password')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.bind_as(base: 'dc=example,dc=com', filter: "(uid=#{username})", password: password)
  BenchmarkResponse.ok('auth complete')
end
