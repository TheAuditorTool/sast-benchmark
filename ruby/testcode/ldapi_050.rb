require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

def validate_uid(raw)
  raise 'invalid uid format' unless raw =~ /\A[a-z][a-z0-9_]{0,31}\z/
  raw
end

# vuln-code-snippet start ruby_ldapi_dry_validation_ldap
def search_dry_validated(req)
  validated_uid = validate_uid(req.param('uid')) # vuln-code-snippet safe-line ruby_ldapi_dry_validation_ldap
  filter = Net::LDAP::Filter.eq('uid', validated_uid)
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_dry_validation_ldap
