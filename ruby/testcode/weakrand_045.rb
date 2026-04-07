require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_otp
def generate_otp(req)
  otp = SecureRandom.random_number(999_999).to_s.rjust(6, '0') # vuln-code-snippet safe-line ruby_weakrand_securerandom_otp
  BenchmarkResponse.json({ otp: otp })
end
# vuln-code-snippet end ruby_weakrand_securerandom_otp
