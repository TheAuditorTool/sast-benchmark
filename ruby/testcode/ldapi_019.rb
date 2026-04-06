require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
end; end

# vuln-code-snippet start ruby_ldapi_format_filter
def search_format(req)
  email = req.param('email')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: "(mail=%s)" % email) # vuln-code-snippet vuln-line ruby_ldapi_format_filter
  BenchmarkResponse.ok('search complete')
end
# vuln-code-snippet end ruby_ldapi_format_filter
