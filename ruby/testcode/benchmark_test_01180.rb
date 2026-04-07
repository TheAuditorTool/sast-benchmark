require_relative 'shared'

def validate_url(req)
  url = req.param('url')
  matched = url.match(%r{\Ahttps?://([a-zA-Z0-9\-]+\.)+[a-zA-Z]{2,}(/\S*)?\z})
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
