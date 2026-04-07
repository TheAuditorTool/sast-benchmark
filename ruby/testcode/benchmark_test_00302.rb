require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.bind_as(opts = {}); true; end
  module Filter
    def self.eq(attr, val); "(#{attr}=#{val})"; end
  end
end; end

def auth_bind_safe(req)
  username = req.param('username')
  password = req.param('password')
  filter = Net::LDAP::Filter.eq('uid', username)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.bind_as(base: 'dc=example,dc=com', filter: filter, password: password)
  BenchmarkResponse.ok('auth complete')
end
