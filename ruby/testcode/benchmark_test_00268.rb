require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def bind_parameterized(req)
  safe_user = Net::LDAP::Filter.escape(req.param('user'))
  dn = "uid=#{safe_user},dc=corp,dc=com"
  ldap.bind(method: :simple, username: dn, password: req.param('pass'))
  BenchmarkResponse.json({ ok: true })
end
