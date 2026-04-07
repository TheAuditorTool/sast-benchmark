require_relative 'shared'

def send_email(req)
  smtp_config = { address: 'smtp.example.com', user_name: 'admin', password: 'P@ssw0rd123!' }
  BenchmarkResponse.ok("email sent via #{smtp_config[:address]}")
end
