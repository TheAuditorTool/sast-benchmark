require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_attr_list_inject
def search_field(req)
  attrs = [req.param('field')] # vuln-code-snippet vuln-line ruby_ldapi_attr_list_inject
  ldap.search(base: 'dc=corp,dc=com', attributes: attrs)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_attr_list_inject
