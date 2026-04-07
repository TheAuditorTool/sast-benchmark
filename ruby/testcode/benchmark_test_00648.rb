require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def validate_uid(raw)
  raise 'invalid uid format' unless raw =~ /\A[a-z][a-z0-9_]{0,31}\z/
  raw
end

def search_dry_validated(req)
  validated_uid = validate_uid(req.param('uid'))
  filter = Net::LDAP::Filter.eq('uid', validated_uid)
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
