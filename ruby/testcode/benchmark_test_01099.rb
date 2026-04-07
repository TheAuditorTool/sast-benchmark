require_relative 'shared'

def return_redirect(req)
  target = req.param('return_to')
  BenchmarkResponse.redirect(target)
end
