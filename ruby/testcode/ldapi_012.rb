require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

# vuln-code-snippet start ruby_ldapi_hardcoded_base
def search_hardcoded(req)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(base: 'ou=users,dc=example,dc=com', filter: '(objectClass=person)') # vuln-code-snippet safe-line ruby_ldapi_hardcoded_base
  BenchmarkResponse.ok('search complete')
end
# vuln-code-snippet end ruby_ldapi_hardcoded_base
