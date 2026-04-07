require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_base_dn_inject
def search_dept(req)
  base = "ou=#{req.param('dept')},dc=corp,dc=com" # vuln-code-snippet vuln-line ruby_ldapi_base_dn_inject
  filter = Net::LDAP::Filter.present('objectClass')
  ldap.search(base: base, filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_base_dn_inject
