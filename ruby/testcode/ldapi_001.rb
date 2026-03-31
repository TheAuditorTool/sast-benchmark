require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.search(options = {}); []; end
  end
end

# vuln-code-snippet start ruby_ldapi_raw_filter
def lookup_user(req)
  username = req.param('username')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  entries = ldap.search(filter: "(uid=#{username})")  # vuln-code-snippet vuln-line ruby_ldapi_raw_filter
  BenchmarkResponse.json({ count: entries.length })
end
# vuln-code-snippet end ruby_ldapi_raw_filter
