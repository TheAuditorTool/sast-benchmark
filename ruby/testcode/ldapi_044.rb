require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_filter_and_eq
def search_active_user(req)
  f1 = Net::LDAP::Filter.eq('uid', req.param('uid'))
  f2 = Net::LDAP::Filter.eq('active', 'TRUE')
  combined = f1 & f2 # vuln-code-snippet safe-line ruby_ldapi_filter_and_eq
  ldap.search(base: 'dc=corp,dc=com', filter: combined)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_filter_and_eq
