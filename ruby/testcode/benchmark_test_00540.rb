require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

def search_group(req)
  group = req.param('group')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: "(&(cn=#{group})(objectClass=groupOfNames))")
  BenchmarkResponse.ok('search complete')
end
