require_relative 'shared'

CORS_ALLOWED_ORIGINS = %w[https://app.example.com https://admin.example.com].freeze

def set_cors_allowlisted(req)
  origin = req.param('origin')
  safe_origin = CORS_ALLOWED_ORIGINS.include?(origin) ? origin : 'null'
  response = BenchmarkResponse.ok('ok')
  response.headers['Access-Control-Allow-Origin'] = safe_origin
  response
end
