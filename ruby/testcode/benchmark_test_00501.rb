require_relative 'shared'

def redirect_to_root(req)
  BenchmarkResponse.redirect('/')
end
