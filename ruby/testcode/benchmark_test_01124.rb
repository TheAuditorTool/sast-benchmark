require_relative 'shared'

def handler(req)
  params = FakeParams.new(req.post_data)
  allowed = params.permit(:name, :email)
  user = FakeActiveRecord::Base.where(allowed.to_h)
  BenchmarkResponse.ok(user.to_a.first.to_s)
end
