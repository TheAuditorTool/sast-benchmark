require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.bind_as(opts = {}); true; end
end; end

# vuln-code-snippet start ruby_ldapi_bind_filter
def auth_bind(req)
  username = req.param('username')
  password = req.param('password')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.bind_as(base: 'dc=example,dc=com', filter: "(uid=#{username})", password: password) # vuln-code-snippet vuln-line ruby_ldapi_bind_filter
  BenchmarkResponse.ok('auth complete')
end
# vuln-code-snippet end ruby_ldapi_bind_filter
