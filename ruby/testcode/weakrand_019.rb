require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_rand_otp
def generate_otp(req)
  otp = rand(1_000_000).to_s.rjust(6, '0') # vuln-code-snippet vuln-line ruby_weakrand_rand_otp
  BenchmarkResponse.json({ otp: otp })
end
# vuln-code-snippet end ruby_weakrand_rand_otp
