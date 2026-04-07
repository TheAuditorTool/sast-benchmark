require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

def search_scope(req)
  scope = req.param('scope')
  base = req.param('base')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(base: base, scope: scope, filter: '(objectClass=*)')
  BenchmarkResponse.ok('search complete')
end
