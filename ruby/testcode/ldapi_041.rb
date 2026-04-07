require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_constant_filter_obj
def search_all_persons(req)
  filter = Net::LDAP::Filter.present('objectClass') # vuln-code-snippet safe-line ruby_ldapi_constant_filter_obj
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_constant_filter_obj
