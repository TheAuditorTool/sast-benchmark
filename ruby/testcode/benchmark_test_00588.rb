require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

def search_format(req)
  email = req.param('email')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: "(mail=%s)" % email)
  BenchmarkResponse.ok('search complete')
end
