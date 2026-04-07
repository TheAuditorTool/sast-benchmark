require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def search_format(req)
  filter_str = "(uid=%s)" % req.param('user')
  filter = Net::LDAP::Filter.construct(filter_str)
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
