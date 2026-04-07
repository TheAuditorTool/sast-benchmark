require_relative 'shared'

def handler(req)
  params = FakeParams.new(req.post_data)
  params.permit!
  user = FakeActiveRecord::Base.where(params.to_h)
  BenchmarkResponse.ok(user.to_a.first.to_s)
end
