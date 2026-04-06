require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

# vuln-code-snippet start ruby_ldapi_constant_filter
def search_constant(req)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  results = ldap.search(filter: '(&(objectClass=person)(ou=engineering))') # vuln-code-snippet safe-line ruby_ldapi_constant_filter
  BenchmarkResponse.ok("found #{results.length}")
end
# vuln-code-snippet end ruby_ldapi_constant_filter
