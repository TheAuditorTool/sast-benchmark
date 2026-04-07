require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end

def check_ldap(req)
  Net::LDAP.new(host: req.param('ldap_host')).bind
  BenchmarkResponse.json({ ok: true })
end
