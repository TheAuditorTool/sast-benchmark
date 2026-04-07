require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.search(options = {}); []; end
  end
end

def get_service_account(req)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  entries = ldap.search(base: 'cn=service,ou=accounts,dc=example,dc=com')
  BenchmarkResponse.json({ count: entries.length })
end
