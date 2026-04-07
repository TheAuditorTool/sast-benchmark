require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def bind_user(req)
  filter = Net::LDAP::Filter.construct("uid=#{req.param('user')}")
  ldap.bind_as(filter: filter, password: req.param('pass'))
  BenchmarkResponse.json({ ok: true })
end
