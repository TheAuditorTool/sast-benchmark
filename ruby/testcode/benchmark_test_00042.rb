require_relative 'shared'

module Net; module LDAP
  def self.new(opts = {}); self; end
  def self.search(opts = {}); []; end
  module Filter
    def self.eq(attr, val); "(#{attr}=#{val})"; end
    def self.escape(val); val.gsub(/[\\*()"\0]/) { |c| "\\%02x" % c.ord }; end
  end
end; end

def search_escaped(req)
  username = req.param('username')
  safe_name = Net::LDAP::Filter.escape(username)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  ldap.search(filter: "(uid=#{safe_name})")
  BenchmarkResponse.ok('search complete')
end
