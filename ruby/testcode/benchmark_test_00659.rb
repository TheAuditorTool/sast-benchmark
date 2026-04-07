require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.search(options = {}); []; end
  end
end

def lookup_user(req)
  username = req.param('username')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  entries = ldap.search(filter: "(uid=#{username})")
  BenchmarkResponse.json({ count: entries.length })
end
