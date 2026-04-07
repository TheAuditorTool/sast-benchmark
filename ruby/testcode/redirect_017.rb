require_relative 'shared'

# vuln-code-snippet start ruby_redirect_callback_param
def redirect_callback_param(req)
  url = req.param('callback')
  BenchmarkResponse.new(302, { 'Location' => url }, '') # vuln-code-snippet vuln-line ruby_redirect_callback_param
end
# vuln-code-snippet end ruby_redirect_callback_param
