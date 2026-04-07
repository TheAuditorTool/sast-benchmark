require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_multiline_log
def set_debug_header(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Debug-Info'] = req.param('debug') # vuln-code-snippet vuln-line ruby_headerinj_multiline_log
  response
end
# vuln-code-snippet end ruby_headerinj_multiline_log
