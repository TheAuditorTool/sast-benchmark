require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_hardcoded_lambda
def apply_discount(req)
  amount = req.param('amount').to_f
  discount = ->(price) { price * 0.9 } # vuln-code-snippet safe-line ruby_codeinj_hardcoded_lambda
  result = discount.call(amount)
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_codeinj_hardcoded_lambda
