require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

FIXED_BASE_DN = 'dc=corp,dc=com'.freeze

# vuln-code-snippet start ruby_ldapi_hardcoded_base_dn
def search_fixed_base(req)
  filter = Net::LDAP::Filter.eq('uid', req.param('user'))
  ldap.search(base: FIXED_BASE_DN, filter: filter) # vuln-code-snippet safe-line ruby_ldapi_hardcoded_base_dn
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_hardcoded_base_dn
