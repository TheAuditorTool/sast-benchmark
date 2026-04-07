require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_parameterized_bind_op
def bind_parameterized(req)
  safe_user = Net::LDAP::Filter.escape(req.param('user'))
  dn = "uid=#{safe_user},dc=corp,dc=com" # vuln-code-snippet safe-line ruby_ldapi_parameterized_bind_op
  ldap.bind(method: :simple, username: dn, password: req.param('pass'))
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_parameterized_bind_op
