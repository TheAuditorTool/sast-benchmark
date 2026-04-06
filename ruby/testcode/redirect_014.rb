require_relative 'shared'

# vuln-code-snippet start ruby_redirect_relative_only
def redirect_relative(req)
  path = req.param('path')
  return BenchmarkResponse.bad_request('absolute not allowed') if path.include?('://') || path.start_with?('//') # vuln-code-snippet safe-line ruby_redirect_relative_only
  BenchmarkResponse.redirect(path)
end
# vuln-code-snippet end ruby_redirect_relative_only
