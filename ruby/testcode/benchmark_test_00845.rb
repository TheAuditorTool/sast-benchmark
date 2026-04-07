require_relative 'shared'

def update_explicit(req)
  name = req.post('name')
  email = req.post('email')
  attrs = { name: name, email: email }
  BenchmarkResponse.ok("updated: #{attrs}")
end
