require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_range_otp
def generate_otp(req)
  otp = rand(100_000..999_999) # vuln-code-snippet vuln-line ruby_weakrand_range_otp
  BenchmarkResponse.ok(otp.to_s)
end
# vuln-code-snippet end ruby_weakrand_range_otp
