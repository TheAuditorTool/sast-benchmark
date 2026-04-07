require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_escape_construct
def search_escaped(req)
  safe = Net::LDAP::Filter.escape(req.param('user')) # vuln-code-snippet safe-line ruby_ldapi_escape_construct
  filter = Net::LDAP::Filter.construct("(uid=#{safe})")
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_escape_construct
