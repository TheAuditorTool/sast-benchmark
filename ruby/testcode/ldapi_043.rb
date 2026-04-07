require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

BIND_DN = 'uid=service,dc=corp,dc=com'.freeze

# vuln-code-snippet start ruby_ldapi_net_escape_bind
def bind_with_escaped_password(req)
  safe_pass = Net::LDAP::Filter.escape(req.param('password')) # vuln-code-snippet safe-line ruby_ldapi_net_escape_bind
  ldap.bind(method: :simple, username: BIND_DN, password: safe_pass)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_net_escape_bind
