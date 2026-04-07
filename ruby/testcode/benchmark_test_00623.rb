require_relative 'shared'

def set_custom_stripped(req)
  val = req.param('value').gsub(/[\r\n\x00]/, '')
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Custom'] = val
  response
end
