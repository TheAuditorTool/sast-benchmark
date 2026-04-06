require_relative 'shared'

# vuln-code-snippet start ruby_csrf_origin_unchecked
def process_payment(req)
  amount = req.post('amount')
  BenchmarkResponse.ok("payment: #{amount}") # vuln-code-snippet vuln-line ruby_csrf_origin_unchecked
end
# vuln-code-snippet end ruby_csrf_origin_unchecked
