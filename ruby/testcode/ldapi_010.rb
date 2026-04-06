require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
  module Filter
    def self.eq(attr, val); "(#{attr}=#{val})"; end
  end
end; end

# vuln-code-snippet start ruby_ldapi_filter_eq_obj
def search_filter_obj(req)
  username = req.param('username')
  filter = Net::LDAP::Filter.eq('uid', username) # vuln-code-snippet safe-line ruby_ldapi_filter_eq_obj
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: filter)
  BenchmarkResponse.ok('search complete')
end
# vuln-code-snippet end ruby_ldapi_filter_eq_obj
