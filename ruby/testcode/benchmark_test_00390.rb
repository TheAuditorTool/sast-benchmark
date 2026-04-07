require_relative 'shared'

def handle_url_validate_redos(req)
  url = req.param('url')
  result = /^(https?:\/\/)([\da-z\.-]+)\.([a-z\.]{2,6})([\/\w \.-]*)*\/?$/.match(url)
  BenchmarkResponse.json({ valid: !result.nil? })
end
