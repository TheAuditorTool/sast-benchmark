require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

# vuln-code-snippet start ruby_ldapi_group_filter
def search_group(req)
  group = req.param('group')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: "(&(cn=#{group})(objectClass=groupOfNames))") # vuln-code-snippet vuln-line ruby_ldapi_group_filter
  BenchmarkResponse.ok('search complete')
end
# vuln-code-snippet end ruby_ldapi_group_filter
