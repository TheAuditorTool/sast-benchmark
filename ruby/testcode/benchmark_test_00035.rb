require_relative 'shared'

def redirect_relative(req)
  path = req.param('path')
  return BenchmarkResponse.bad_request('absolute not allowed') if path.include?('://') || path.start_with?('//')
  BenchmarkResponse.redirect(path)
end
