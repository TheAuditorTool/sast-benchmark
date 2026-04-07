require_relative 'shared'

def set_preferences_cookie(req)
  prefs = req.post('preferences')
  resp = BenchmarkResponse.ok('saved')
  resp.set_cookie('prefs', prefs, path: '/', expires: Time.now + 86400 * 30, secure: true, httponly: true)
  resp
end
