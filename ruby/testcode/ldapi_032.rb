require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def build_uid_filter(q)
  "(uid=#{q})"
end

# vuln-code-snippet start ruby_ldapi_multihop_filter
def search_multihop(req)
  f = Net::LDAP::Filter.construct(build_uid_filter(req.param('q'))) # vuln-code-snippet vuln-line ruby_ldapi_multihop_filter
  ldap.search(base: 'dc=corp,dc=com', filter: f)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_multihop_filter
