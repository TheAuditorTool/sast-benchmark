require_relative 'shared'

ALLOWED_ATTRS = %w[uid cn mail givenName sn].freeze

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

def search_allowlist(req)
  attr_name = req.param('attr')
  return BenchmarkResponse.bad_request('invalid attr') unless ALLOWED_ATTRS.include?(attr_name)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: '(objectClass=person)', attributes: [attr_name])
  BenchmarkResponse.ok('search complete')
end
