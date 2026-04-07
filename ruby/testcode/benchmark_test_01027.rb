require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def search_by_validated_email(req)
  email = req.param('email')
  raise 'invalid email' unless email =~ /\A[^@]+@[^@]+\.[^@]+\z/
  filter = Net::LDAP::Filter.eq('mail', email)
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
