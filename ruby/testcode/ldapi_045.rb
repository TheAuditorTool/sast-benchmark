require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

SAFE_LDAP_ATTRS = %w[cn mail uid sn].freeze

# vuln-code-snippet start ruby_ldapi_attr_from_allowlist
def search_safe_attrs(req)
  attrs = SAFE_LDAP_ATTRS & req.param('attrs').split(',') # vuln-code-snippet safe-line ruby_ldapi_attr_from_allowlist
  ldap.search(base: 'dc=corp,dc=com', attributes: attrs)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_attr_from_allowlist
