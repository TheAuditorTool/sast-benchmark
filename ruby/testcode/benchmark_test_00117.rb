require_relative 'shared'

def callback_redirect(req)
  domain = req.param('domain')
  BenchmarkResponse.redirect("https://" + domain + "/callback")
end
