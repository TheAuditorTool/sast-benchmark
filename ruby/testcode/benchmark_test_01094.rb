require_relative 'shared'

def generate_identifier(req)
  _name = req.param('name')
  uid = SecureRandom.uuid
  BenchmarkResponse.ok(uid)
end
