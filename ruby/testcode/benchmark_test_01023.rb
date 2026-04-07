require_relative 'shared'

def set_ascii_printable_header(req)
  sanitized = req.param('value').gsub(/[^\x20-\x7E]/, '')
  response = BenchmarkResponse.ok('ok')
  response.headers['X-App-Data'] = sanitized
  response
end
