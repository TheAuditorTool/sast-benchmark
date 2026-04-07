require_relative 'shared'

def apply_discount(req)
  amount = req.param('amount').to_f
  discount = ->(price) { price * 0.9 }
  result = discount.call(amount)
  BenchmarkResponse.ok(result.to_s)
end
