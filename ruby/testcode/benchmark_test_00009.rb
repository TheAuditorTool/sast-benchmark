require_relative 'shared'

def generate_otp(req)
  otp = rand(100_000..999_999)
  BenchmarkResponse.ok(otp.to_s)
end
