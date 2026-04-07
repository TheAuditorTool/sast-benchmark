require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_rename_taint
def rename_entry(req)
  olddn = "cn=#{req.param('old')},dc=corp,dc=com" # vuln-code-snippet vuln-line ruby_ldapi_rename_taint
  newrdn = "cn=#{req.param('new')}"
  ldap.rename(olddn: olddn, newrdn: newrdn)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_rename_taint
