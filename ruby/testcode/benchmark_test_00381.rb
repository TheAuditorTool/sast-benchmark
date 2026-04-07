require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def search_with_scope(req)
  ldap.search(base: 'dc=corp,dc=com', scope: req.param('scope').to_i)
  BenchmarkResponse.json({ ok: true })
end
