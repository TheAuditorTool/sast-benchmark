require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

def search_validated(req)
  username = req.param('username')
  return BenchmarkResponse.bad_request('invalid') unless username.match?(/\A[a-zA-Z0-9._-]+\z/)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: "(uid=#{username})")
  BenchmarkResponse.ok('search complete')
end
