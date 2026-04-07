require_relative 'shared'

def handler(req)
  result = FakeActiveRecord::Base.where(req.post_data)
  BenchmarkResponse.ok(result.to_a.first.to_s)
end
