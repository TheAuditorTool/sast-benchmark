require_relative 'shared'

def api_update_safe(req)
  cookie_token = req.cookie('csrf_token')
  header_token = req.header('X-CSRF-Token')
  unless cookie_token && header_token && Rack::Utils.secure_compare(cookie_token, header_token)
    return BenchmarkResponse.bad_request('CSRF validation failed')
  end
  data = req.post('data')
  BenchmarkResponse.json({ result: data })
end
