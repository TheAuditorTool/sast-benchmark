require_relative 'shared'

def fixed_redirect(req)
  _input = req.param('url')
  BenchmarkResponse.redirect('/')
end
