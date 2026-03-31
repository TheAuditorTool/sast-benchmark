require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.search(options = {}); []; end
  end
end

# vuln-code-snippet start ruby_ldapi_search_concat
def find_person(req)
  name = req.param('name')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  entries = ldap.search(filter: "(&(objectClass=person)(cn=" + name + "))")  # vuln-code-snippet vuln-line ruby_ldapi_search_concat
  BenchmarkResponse.json({ count: entries.length })
end
# vuln-code-snippet end ruby_ldapi_search_concat
