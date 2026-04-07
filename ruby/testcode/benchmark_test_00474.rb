require_relative 'shared'

def set_content_type(req)
  ctype = req.param('type')
  BenchmarkResponse.new(200, 'ok', { 'Content-Type' => ctype })
end
