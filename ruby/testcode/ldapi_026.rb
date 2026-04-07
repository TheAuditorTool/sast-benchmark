require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_add_dn_taint
def add_user(req)
  username = req.param('username')
  dn = "uid=#{username},ou=people,dc=corp,dc=com" # vuln-code-snippet vuln-line ruby_ldapi_add_dn_taint
  ldap.add(dn: dn, attributes: { cn: username })
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_add_dn_taint
