require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.bind(options = {}); true; end
  end
end

# vuln-code-snippet start ruby_ldapi_dn_interp
def authenticate_user(req)
  user = req.param('username')
  password = req.param('password')
  base_dn = 'dc=example,dc=com'
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.bind(method: :simple, username: "uid=#{user},#{base_dn}", password: password)  # vuln-code-snippet vuln-line ruby_ldapi_dn_interp
  BenchmarkResponse.ok('authenticated')
end
# vuln-code-snippet end ruby_ldapi_dn_interp
