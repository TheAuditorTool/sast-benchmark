require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.modify(opts = {}); true; end
end; end

def modify_entry(req)
  cn = req.param('cn')
  email = req.param('email')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.modify(dn: "cn=#{cn},ou=users,dc=example,dc=com", operations: [[:replace, :mail, email]])
  BenchmarkResponse.ok('modified')
end
