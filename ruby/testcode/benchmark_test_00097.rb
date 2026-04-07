require_relative 'shared'

def handler(req)
  params = FakeParams.new(req.post_data)
  user = { id: 1 }
  user[:name] = params[:name]
  user[:email] = params[:email]
  BenchmarkResponse.ok(user.to_s)
end
