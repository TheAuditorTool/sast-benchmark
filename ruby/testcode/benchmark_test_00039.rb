require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def search_active_user(req)
  f1 = Net::LDAP::Filter.eq('uid', req.param('uid'))
  f2 = Net::LDAP::Filter.eq('active', 'TRUE')
  combined = f1 & f2
  ldap.search(base: 'dc=corp,dc=com', filter: combined)
  BenchmarkResponse.json({ ok: true })
end
