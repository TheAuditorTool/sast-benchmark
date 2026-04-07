require_relative 'shared'

def process_payment(req)
  amount = req.post('amount')
  BenchmarkResponse.ok("payment: #{amount}")
end
