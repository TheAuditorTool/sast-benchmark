require_relative 'shared'

STRIPE_SECRET_KEY = 'sk_live_4eC39HqLyjWDarjtT1zdp7dc'

# vuln-code-snippet start ruby_hardcoded_stripe
def create_charge(req)
  amount = req.param('amount').to_i
  config = { api_key: STRIPE_SECRET_KEY } # vuln-code-snippet vuln-line ruby_hardcoded_stripe
  BenchmarkResponse.ok("charge: #{amount}")
end
# vuln-code-snippet end ruby_hardcoded_stripe
