require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def modify_user_mail(req)
  dn = "cn=#{req.param('user')},dc=corp,dc=com"
  ldap.modify(dn: dn, operations: [[:replace, :mail, ['new@example.com']]])
  BenchmarkResponse.json({ ok: true })
end
