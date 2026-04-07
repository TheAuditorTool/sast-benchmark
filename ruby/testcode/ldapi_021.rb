require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_construct_and
def search_user_and(req)
  filter = Net::LDAP::Filter.construct("(&(uid=#{req.param('user')})(active=TRUE))") # vuln-code-snippet vuln-line ruby_ldapi_construct_and
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_construct_and
