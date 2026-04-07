require_relative 'shared'

def handler(req)
  safe_data = req.post_data.slice('name', 'email')
  FakeActiveRecord::Base.where(safe_data)
  BenchmarkResponse.ok('saved')
end
