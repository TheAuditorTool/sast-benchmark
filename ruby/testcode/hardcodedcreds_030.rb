require 'net/smtp'
require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_smtp_pass
def send_email_handler(req)
  smtp_password = 'MyEmailP@ssw0rd'
  Net::SMTP.start('smtp.example.com', 587, 'localhost', 'user@example.com', smtp_password, :login)  # vuln-code-snippet vuln-line ruby_hardcoded_smtp_pass
  BenchmarkResponse.ok('email sent')
end
# vuln-code-snippet end ruby_hardcoded_smtp_pass
