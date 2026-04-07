require_relative 'shared'
require 'securerandom'

def generate_otp(req)
  otp = SecureRandom.random_number(999_999).to_s.rjust(6, '0')
  BenchmarkResponse.json({ otp: otp })
end
