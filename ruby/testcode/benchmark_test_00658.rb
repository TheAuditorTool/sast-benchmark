require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

module ActiveLdapStub
  def self.find(cn)
    ldap = Net::LDAP.new(host: 'ldap.corp.com') rescue nil
    ldap.search(base: "cn=#{cn},dc=corp,dc=com") rescue nil
  end
end

def find_by_cn(req)
  ActiveLdapStub.find(req.param('cn'))
  BenchmarkResponse.json({ ok: true })
end
