require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.modify(opts = {}); true; end
end; end

# vuln-code-snippet start ruby_ldapi_modify_dn
def modify_entry(req)
  cn = req.param('cn')
  email = req.param('email')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.modify(dn: "cn=#{cn},ou=users,dc=example,dc=com", operations: [[:replace, :mail, email]]) # vuln-code-snippet vuln-line ruby_ldapi_modify_dn
  BenchmarkResponse.ok('modified')
end
# vuln-code-snippet end ruby_ldapi_modify_dn
