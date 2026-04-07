require_relative 'shared'

def handler(req)
  params = FakeParams.new(req.post_data)
  user_params = params.require(:user).permit(:name)
  FakeActiveRecord::Base.where(user_params.to_h)
  BenchmarkResponse.ok('saved')
end
