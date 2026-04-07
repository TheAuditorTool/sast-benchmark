require_relative 'shared'

CORS_CONFIG = { origins: ['https://app.example.com'] }.freeze

# vuln-code-snippet start ruby_headerinj_cors_gem
def set_cors_via_config(req)
  origin = req.param('origin')
  allowed = CORS_CONFIG[:origins].include?(origin) ? origin : 'null' # vuln-code-snippet safe-line ruby_headerinj_cors_gem
  response = BenchmarkResponse.ok('ok')
  response.headers['Access-Control-Allow-Origin'] = allowed
  response
end
# vuln-code-snippet end ruby_headerinj_cors_gem
