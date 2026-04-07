require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_integer_retry
def set_retry_after_clamped(req)
  secs = Integer(req.param('secs')).clamp(0, 3600).to_s # vuln-code-snippet safe-line ruby_headerinj_integer_retry
  response = BenchmarkResponse.ok('ok')
  response.headers['Retry-After'] = secs
  response
end
# vuln-code-snippet end ruby_headerinj_integer_retry
