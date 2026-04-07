require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_filter_eq_mail
def search_by_email(req)
  filter = Net::LDAP::Filter.eq('mail', req.param('email')) # vuln-code-snippet safe-line ruby_ldapi_filter_eq_mail
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_filter_eq_mail
