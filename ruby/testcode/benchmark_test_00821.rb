require_relative 'shared'

def redirect_protocol(req)
  domain = req.param('domain')
  BenchmarkResponse.redirect("//#{domain}/callback")
end
