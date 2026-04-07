require_relative 'shared'

def open_redirect(req)
  url = req.param('url')
  BenchmarkResponse.redirect(url)
end
