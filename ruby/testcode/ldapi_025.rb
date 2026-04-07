require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_modify_dn_taint
def modify_user_mail(req)
  dn = "cn=#{req.param('user')},dc=corp,dc=com" # vuln-code-snippet vuln-line ruby_ldapi_modify_dn_taint
  ldap.modify(dn: dn, operations: [[:replace, :mail, ['new@example.com']]])
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_modify_dn_taint
