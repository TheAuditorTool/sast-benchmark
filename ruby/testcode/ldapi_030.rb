require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_format_construct
def search_format(req)
  filter_str = "(uid=%s)" % req.param('user') # vuln-code-snippet vuln-line ruby_ldapi_format_construct
  filter = Net::LDAP::Filter.construct(filter_str)
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_format_construct
