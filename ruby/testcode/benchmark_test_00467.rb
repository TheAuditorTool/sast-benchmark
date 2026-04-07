require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def search_field(req)
  attrs = [req.param('field')]
  ldap.search(base: 'dc=corp,dc=com', attributes: attrs)
  BenchmarkResponse.json({ ok: true })
end
