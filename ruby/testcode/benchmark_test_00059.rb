require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def search_group_members(req)
  filter = Net::LDAP::Filter.construct("(memberOf=cn=#{req.param('group')},ou=groups,dc=corp,dc=com)")
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
