require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

FIXED_BASE_DN = 'dc=corp,dc=com'.freeze

def search_fixed_base(req)
  filter = Net::LDAP::Filter.eq('uid', req.param('user'))
  ldap.search(base: FIXED_BASE_DN, filter: filter)
  BenchmarkResponse.json({ ok: true })
end
