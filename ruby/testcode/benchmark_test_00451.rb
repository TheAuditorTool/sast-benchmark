require_relative 'shared'

STRIPE_SECRET_KEY = 'sk_live_4eC39HqLyjWDarjtT1zdp7dc'

def create_charge(req)
  amount = req.param('amount').to_i
  config = { api_key: STRIPE_SECRET_KEY }
  BenchmarkResponse.ok("charge: #{amount}")
end
