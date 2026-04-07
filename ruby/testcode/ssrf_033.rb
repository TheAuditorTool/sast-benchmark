require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end

# vuln-code-snippet start ruby_ssrf_ldap_host
def check_ldap(req)
  Net::LDAP.new(host: req.param('ldap_host')).bind # vuln-code-snippet vuln-line ruby_ssrf_ldap_host
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_ldap_host
