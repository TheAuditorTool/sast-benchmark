require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

# vuln-code-snippet start ruby_ldapi_scope_param
def search_scope(req)
  scope = req.param('scope')
  base = req.param('base')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(base: base, scope: scope, filter: '(objectClass=*)') # vuln-code-snippet vuln-line ruby_ldapi_scope_param
  BenchmarkResponse.ok('search complete')
end
# vuln-code-snippet end ruby_ldapi_scope_param
