require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

LDAP_SCOPE = Net::LDAP::SearchScope_WholeSubtree rescue 2

def search_whole_subtree(req)
  filter = Net::LDAP::Filter.eq('uid', req.param('uid'))
  ldap.search(base: 'dc=corp,dc=com', scope: LDAP_SCOPE, filter: filter)
  BenchmarkResponse.json({ ok: true })
end
