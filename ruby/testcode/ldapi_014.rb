require_relative 'shared'

ALLOWED_ATTRS = %w[uid cn mail givenName sn].freeze

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

# vuln-code-snippet start ruby_ldapi_allowlist_attr
def search_allowlist(req)
  attr_name = req.param('attr')
  return BenchmarkResponse.bad_request('invalid attr') unless ALLOWED_ATTRS.include?(attr_name) # vuln-code-snippet safe-line ruby_ldapi_allowlist_attr
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: '(objectClass=person)', attributes: [attr_name])
  BenchmarkResponse.ok('search complete')
end
# vuln-code-snippet end ruby_ldapi_allowlist_attr
