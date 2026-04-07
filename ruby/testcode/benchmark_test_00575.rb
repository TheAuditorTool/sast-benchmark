require_relative 'shared'
require 'timeout'

def handle_timeout_block(req)
  input = req.param('input')
  result = Timeout.timeout(1) { /\A[a-z]+(\s[a-z]+)*\z/.match(input) }
  BenchmarkResponse.json({ matched: !result.nil? })
end
