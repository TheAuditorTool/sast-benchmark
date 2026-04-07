require_relative 'shared'

# vuln-code-snippet start ruby_dynmethod_pattern_match
def pattern_match_dispatch(req)
  result = case req.param('action') # vuln-code-snippet safe-line ruby_dynmethod_pattern_match
           in 'activate'   then 'activated'
           in 'deactivate' then 'deactivated'
           else raise ArgumentError, 'unknown action'
           end
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_pattern_match
