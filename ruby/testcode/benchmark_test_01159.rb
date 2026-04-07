require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def search_user_or(req)
  q = req.param('q')
  filter = Net::LDAP::Filter.construct("(|(uid=#{q})(mail=#{q}))")
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
