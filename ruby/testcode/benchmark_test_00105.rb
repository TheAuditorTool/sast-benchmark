require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

def search_attrs(req)
  attr_name = req.param('attr')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: '(objectClass=person)', attributes: [attr_name])
  BenchmarkResponse.ok('search complete')
end
