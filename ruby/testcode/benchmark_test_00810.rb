require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

def search_hardcoded(req)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(base: 'ou=users,dc=example,dc=com', filter: '(objectClass=person)')
  BenchmarkResponse.ok('search complete')
end
