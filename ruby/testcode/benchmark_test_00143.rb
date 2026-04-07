require_relative 'shared'

require 'rack/utils'

def transfer_funds(req)
  expected = req.cookie('csrf_token')
  provided = req.post('csrf_token')
  verified = Rack::Utils.secure_compare(expected, provided)
  return BenchmarkResponse.error('forbidden', 403) unless verified
  amount = req.post('amount').to_f
  BenchmarkResponse.ok("transferred #{amount}")
end
