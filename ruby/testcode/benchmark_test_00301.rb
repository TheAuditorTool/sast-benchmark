require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def rename_entry(req)
  olddn = "cn=#{req.param('old')},dc=corp,dc=com"
  newrdn = "cn=#{req.param('new')}"
  ldap.rename(olddn: olddn, newrdn: newrdn)
  BenchmarkResponse.json({ ok: true })
end
