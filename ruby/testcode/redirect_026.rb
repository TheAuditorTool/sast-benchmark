require_relative 'shared'

# vuln-code-snippet start ruby_redirect_sinatra_indirect
def redirect_sinatra_indirect(req)
  BenchmarkResponse.new(302, { 'Location' => req.param('destination') }, '') # vuln-code-snippet vuln-line ruby_redirect_sinatra_indirect
end
# vuln-code-snippet end ruby_redirect_sinatra_indirect
