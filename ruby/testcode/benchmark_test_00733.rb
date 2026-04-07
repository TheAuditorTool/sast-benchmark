require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def add_user(req)
  username = req.param('username')
  dn = "uid=#{username},ou=people,dc=corp,dc=com"
  ldap.add(dn: dn, attributes: { cn: username })
  BenchmarkResponse.json({ ok: true })
end
