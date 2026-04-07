require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

LDAP_SCOPE = Net::LDAP::SearchScope_WholeSubtree rescue 2

# vuln-code-snippet start ruby_ldapi_scope_constant
def search_whole_subtree(req)
  filter = Net::LDAP::Filter.eq('uid', req.param('uid'))
  ldap.search(base: 'dc=corp,dc=com', scope: LDAP_SCOPE, filter: filter) # vuln-code-snippet safe-line ruby_ldapi_scope_constant
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_scope_constant
