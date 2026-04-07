require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def delete_entry(req)
  dn = "cn=#{req.param('entry')},dc=corp,dc=com"
  ldap.delete(dn: dn)
  BenchmarkResponse.json({ ok: true })
end
