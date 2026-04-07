require_relative 'shared'

def redirect_concat(req)
  host = req.param('host')
  BenchmarkResponse.redirect("https://#{host}/callback")
end
