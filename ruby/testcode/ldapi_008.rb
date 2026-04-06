require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
  module Filter
    def self.eq(attr, val); "(#{attr}=#{val})"; end
    def self.escape(val); val.gsub(/[\\*()"\0]/) { |c| "\\%02x" % c.ord }; end
  end
end; end

# vuln-code-snippet start ruby_ldapi_net_escape
def search_escaped(req)
  username = req.param('username')
  safe_name = Net::LDAP::Filter.escape(username) # vuln-code-snippet safe-line ruby_ldapi_net_escape
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: "(uid=#{safe_name})")
  BenchmarkResponse.ok('search complete')
end
# vuln-code-snippet end ruby_ldapi_net_escape
