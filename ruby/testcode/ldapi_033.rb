require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_wildcard_inject
def search_wildcard(req)
  filter = Net::LDAP::Filter.construct("(cn=#{req.param('search')}*)") # vuln-code-snippet vuln-line ruby_ldapi_wildcard_inject
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_wildcard_inject
