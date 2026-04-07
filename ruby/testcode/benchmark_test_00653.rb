require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.bind(options = {}); true; end
  end
end

def authenticate_user(req)
  user = req.param('username')
  password = req.param('password')
  base_dn = 'dc=example,dc=com'
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.bind(method: :simple, username: "uid=#{user},#{base_dn}", password: password)
  BenchmarkResponse.ok('authenticated')
end
