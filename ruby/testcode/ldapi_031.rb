require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_group_membership
def search_group_members(req)
  filter = Net::LDAP::Filter.construct("(memberOf=cn=#{req.param('group')},ou=groups,dc=corp,dc=com)") # vuln-code-snippet vuln-line ruby_ldapi_group_membership
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_group_membership
