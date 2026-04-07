require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_scope_from_param
def search_with_scope(req)
  ldap.search(base: 'dc=corp,dc=com', scope: req.param('scope').to_i) # vuln-code-snippet vuln-line ruby_ldapi_scope_from_param
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_scope_from_param
