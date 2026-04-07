require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

def search_constant(req)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  results = ldap.search(filter: '(&(objectClass=person)(ou=engineering))')
  BenchmarkResponse.ok("found #{results.length}")
end
