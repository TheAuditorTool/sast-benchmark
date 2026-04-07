require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.search(options = {}); []; end
  end
end

def find_person(req)
  name = req.param('name')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  entries = ldap.search(filter: "(&(objectClass=person)(cn=" + name + "))")
  BenchmarkResponse.json({ count: entries.length })
end
