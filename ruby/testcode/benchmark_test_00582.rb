require_relative 'shared'

FIXED_CONTENT_TYPE = 'application/json; charset=utf-8'.freeze

def set_fixed_content_type(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Type'] = FIXED_CONTENT_TYPE
  response
end
