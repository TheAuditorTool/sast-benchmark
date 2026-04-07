require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_actiondispatch_safe
def set_ascii_printable_header(req)
  sanitized = req.param('value').gsub(/[^\x20-\x7E]/, '') # vuln-code-snippet safe-line ruby_headerinj_actiondispatch_safe
  response = BenchmarkResponse.ok('ok')
  response.headers['X-App-Data'] = sanitized
  response
end
# vuln-code-snippet end ruby_headerinj_actiondispatch_safe
