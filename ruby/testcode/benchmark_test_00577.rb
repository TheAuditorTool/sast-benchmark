require_relative 'shared'

def handler(req)
  params = FakeParams.new(req.post_data)
  all_params = params.to_unsafe_h
  FakeActiveRecord::Base.where(all_params)
  BenchmarkResponse.ok('saved')
end
