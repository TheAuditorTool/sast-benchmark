require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
  def self.bind_as(opts = {}); true; end
  def self.modify(opts = {}); true; end
  module Filter
    def self.eq(attr, val); "(#{attr}=#{val})"; end
    def self.escape(val); val.gsub(/[\\*()"\0]/) { |c| "\\%02x" % c.ord }; end
  end
end; end

# vuln-code-snippet start ruby_ldapi_base_concat
def search_with_base(req)
  ou = req.param('ou')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(base: "ou=#{ou},dc=example,dc=com", filter: '(objectClass=*)') # vuln-code-snippet vuln-line ruby_ldapi_base_concat
  BenchmarkResponse.ok('search complete')
end
# vuln-code-snippet end ruby_ldapi_base_concat
