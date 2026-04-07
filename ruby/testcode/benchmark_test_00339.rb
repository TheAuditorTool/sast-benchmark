require_relative 'shared'

module Net
  module LDAP
    def self.new(options = {}); options; end
    def self.search(options = {}); []; end
    module Filter
      def self.escape(str)
        str.gsub(/[\\\*\(\)\x00]/) { |c| "\\#{c.unpack1('H2')}" }
      end
    end
  end
end

def search_users(req)
  name = req.param('name')
  safe_name = Net::LDAP::Filter.escape(name)
  ldap = Net::LDAP.new(host: 'ldap.example.com')
  entries = ldap.search(filter: "(cn=#{safe_name})")
  BenchmarkResponse.json({ count: entries.length })
end
