require 'net/smtp'
require_relative 'shared'

def send_email_handler(req)
  smtp_password = 'MyEmailP@ssw0rd'
  Net::SMTP.start('smtp.example.com', 587, 'localhost', 'user@example.com', smtp_password, :login)
  BenchmarkResponse.ok('email sent')
end
