require_relative 'shared'

require 'rack/utils'

# vuln-code-snippet start ruby_csrf_hash_equals
def transfer_funds(req)
  expected = req.cookie('csrf_token')
  provided = req.post('csrf_token')
  verified = Rack::Utils.secure_compare(expected, provided)  # vuln-code-snippet safe-line ruby_csrf_hash_equals
  return BenchmarkResponse.error('forbidden', 403) unless verified
  amount = req.post('amount').to_f
  BenchmarkResponse.ok("transferred #{amount}")
end
# vuln-code-snippet end ruby_csrf_hash_equals
