require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_integer_empno
def search_by_empno(req)
  filter = Net::LDAP::Filter.eq('employeeNumber', Integer(req.param('id')).to_s) # vuln-code-snippet safe-line ruby_ldapi_integer_empno
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_integer_empno
