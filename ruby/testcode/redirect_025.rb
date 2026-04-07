require_relative 'shared'

# vuln-code-snippet start ruby_redirect_returnto_session_unvalidated
def redirect_returnto_session_unvalidated(req)
  return_to = req.param('return_to')
  session_return_to = return_to
  BenchmarkResponse.new(302, { 'Location' => session_return_to }, '') # vuln-code-snippet vuln-line ruby_redirect_returnto_session_unvalidated
end
# vuln-code-snippet end ruby_redirect_returnto_session_unvalidated
