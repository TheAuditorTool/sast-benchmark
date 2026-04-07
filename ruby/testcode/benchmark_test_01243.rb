require_relative 'shared'

CORS_CONFIG = { origins: ['https://app.example.com'] }.freeze

def set_cors_via_config(req)
  origin = req.param('origin')
  allowed = CORS_CONFIG[:origins].include?(origin) ? origin : 'null'
  response = BenchmarkResponse.ok('ok')
  response.headers['Access-Control-Allow-Origin'] = allowed
  response
end
