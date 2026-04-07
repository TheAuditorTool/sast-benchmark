require_relative 'shared'

def update_with_wrong_compare(req)
  token = req.post('csrf_token')
  if token
    BenchmarkResponse.ok('updated')
  else
    BenchmarkResponse.error('CSRF rejected', 403)
  end
end
