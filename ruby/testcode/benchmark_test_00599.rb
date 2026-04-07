require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

SAFE_LDAP_ATTRS = %w[cn mail uid sn].freeze

def search_safe_attrs(req)
  attrs = SAFE_LDAP_ATTRS & req.param('attrs').split(',')
  ldap.search(base: 'dc=corp,dc=com', attributes: attrs)
  BenchmarkResponse.json({ ok: true })
end
