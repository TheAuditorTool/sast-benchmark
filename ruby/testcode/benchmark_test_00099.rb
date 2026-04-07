require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

ALLOWED_DEPTS = %w[engineering sales support].freeze

def search_dept_allowlisted(req)
  d = req.param('dept')
  raise 'invalid department' unless ALLOWED_DEPTS.include?(d)
  ldap.search(base: "ou=#{d},dc=corp,dc=com")
  BenchmarkResponse.json({ ok: true })
end
