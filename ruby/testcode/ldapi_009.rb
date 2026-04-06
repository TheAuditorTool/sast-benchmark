require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

# vuln-code-snippet start ruby_ldapi_attrs_inject
def search_attrs(req)
  attr_name = req.param('attr')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: '(objectClass=person)', attributes: [attr_name]) # vuln-code-snippet vuln-line ruby_ldapi_attrs_inject
  BenchmarkResponse.ok('search complete')
end
# vuln-code-snippet end ruby_ldapi_attrs_inject
