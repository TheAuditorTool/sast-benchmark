require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_delete_taint
def delete_entry(req)
  dn = "cn=#{req.param('entry')},dc=corp,dc=com" # vuln-code-snippet vuln-line ruby_ldapi_delete_taint
  ldap.delete(dn: dn)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_delete_taint
