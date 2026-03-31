require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.search(options = {}); []; end
  end
end

# vuln-code-snippet start ruby_ldapi_hardcoded_dn
def get_service_account(req)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  entries = ldap.search(base: 'cn=service,ou=accounts,dc=example,dc=com')  # vuln-code-snippet safe-line ruby_ldapi_hardcoded_dn
  BenchmarkResponse.json({ count: entries.length })
end
# vuln-code-snippet end ruby_ldapi_hardcoded_dn
