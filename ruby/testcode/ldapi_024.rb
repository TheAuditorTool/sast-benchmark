require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_bind_filter_taint
def bind_user(req)
  filter = Net::LDAP::Filter.construct("uid=#{req.param('user')}") # vuln-code-snippet vuln-line ruby_ldapi_bind_filter_taint
  ldap.bind_as(filter: filter, password: req.param('pass'))
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_bind_filter_taint
