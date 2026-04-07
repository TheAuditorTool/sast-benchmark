require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def build_uid_filter(q)
  "(uid=#{q})"
end

def search_multihop(req)
  f = Net::LDAP::Filter.construct(build_uid_filter(req.param('q')))
  ldap.search(base: 'dc=corp,dc=com', filter: f)
  BenchmarkResponse.json({ ok: true })
end
