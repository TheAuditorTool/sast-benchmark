require 'net/smtp'
require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_encrypted_creds_file
def send_email_safe_handler(req)
  smtp_pass = Rails.application.credentials.smtp_pass  # vuln-code-snippet safe-line ruby_hardcoded_encrypted_creds_file
  Net::SMTP.start('smtp.example.com', 587, 'localhost', 'user@example.com', smtp_pass, :login)
  BenchmarkResponse.ok('email sent')
end
# vuln-code-snippet end ruby_hardcoded_encrypted_creds_file
