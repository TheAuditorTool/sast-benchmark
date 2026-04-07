require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

DN_MAP = {
  1 => 'uid=alice,dc=corp,dc=com',
  2 => 'uid=bob,dc=corp,dc=com'
}.freeze

def lookup_dn_from_db(user_id)
  DN_MAP.fetch(user_id)
end

# vuln-code-snippet start ruby_ldapi_no_user_in_dn
def search_by_db_dn(req)
  user_id = Integer(req.param('id'))
  dn = lookup_dn_from_db(user_id) # vuln-code-snippet safe-line ruby_ldapi_no_user_in_dn
  ldap.search(base: dn)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_no_user_in_dn
