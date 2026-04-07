require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def search_validated_uid(req)
  uid = req.param('uid')
  raise 'invalid uid' unless uid =~ /\A[a-z][a-z0-9_]{0,31}\z/
  filter = Net::LDAP::Filter.eq('uid', uid)
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
