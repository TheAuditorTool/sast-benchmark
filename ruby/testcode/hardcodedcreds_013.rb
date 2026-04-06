require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_smtp
def send_email(req)
  smtp_config = { address: 'smtp.example.com', user_name: 'admin', password: 'P@ssw0rd123!' } # vuln-code-snippet vuln-line ruby_hardcoded_smtp
  BenchmarkResponse.ok("email sent via #{smtp_config[:address]}")
end
# vuln-code-snippet end ruby_hardcoded_smtp
