require_relative 'shared'

begin; require 'net/ldap'; rescue LoadError; end
ldap = Net::LDAP.new(host: 'ldap.corp.com', port: 389) rescue nil

# vuln-code-snippet start ruby_ldapi_email_format_check
def search_by_validated_email(req)
  email = req.param('email')
  raise 'invalid email' unless email =~ /\A[^@]+@[^@]+\.[^@]+\z/ # vuln-code-snippet safe-line ruby_ldapi_email_format_check
  filter = Net::LDAP::Filter.eq('mail', email)
  ldap.search(base: 'dc=corp,dc=com', filter: filter)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ldapi_email_format_check
