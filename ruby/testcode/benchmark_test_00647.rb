require_relative 'shared'

def middleware_strip_crlf(req)
  val = req.header('HTTP_X_CUSTOM').to_s.gsub(/[\r\n]/, '')
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Custom'] = val
  response
end
