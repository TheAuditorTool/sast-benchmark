require_relative 'shared'

def jsonp_api(req)
  callback = req.param('callback')
  csrf_token = req.cookie('csrf_token')
  response_body = "#{callback}({\"csrf_token\":\"#{csrf_token}\",\"user\":\"admin\"})"
  BenchmarkResponse.json({ result: response_body })
end
