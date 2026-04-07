require_relative 'shared'

def generate_otp(req)
  otp = rand(1_000_000).to_s.rjust(6, '0')
  BenchmarkResponse.json({ otp: otp })
end
