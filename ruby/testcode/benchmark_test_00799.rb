require 'net/smtp'
require_relative 'shared'

def send_email_safe_handler(req)
  smtp_pass = Rails.application.credentials.smtp_pass
  Net::SMTP.start('smtp.example.com', 587, 'localhost', 'user@example.com', smtp_pass, :login)
  BenchmarkResponse.ok('email sent')
end
