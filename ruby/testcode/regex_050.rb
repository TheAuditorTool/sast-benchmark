require_relative 'shared'
require 'timeout'

# vuln-code-snippet start ruby_regex_timeout_block
def handle_timeout_block(req)
  input = req.param('input')
  result = Timeout.timeout(1) { /\A[a-z]+(\s[a-z]+)*\z/.match(input) } # vuln-code-snippet safe-line ruby_regex_timeout_block
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_timeout_block
