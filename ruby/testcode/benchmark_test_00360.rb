require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.search(options = {}); []; end
    module Filter
      def self.eq(attr, value); "(#{attr}=#{value})"; end
    end
  end
end

def lookup_user_filtered(req)
  username = req.param('username')
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  safe_filter = Net::LDAP::Filter.eq('uid', username)
  entries = ldap.search(filter: safe_filter)
  BenchmarkResponse.json({ count: entries.length })
end
