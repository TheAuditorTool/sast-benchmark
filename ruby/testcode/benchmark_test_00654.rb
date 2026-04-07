require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def search_dept(req)
  base = "ou=#{req.param('dept')},dc=corp,dc=com"
  filter = Net::LDAP::Filter.present('objectClass')
  ldap.search(base: base, filter: filter)
  BenchmarkResponse.json({ ok: true })
end
