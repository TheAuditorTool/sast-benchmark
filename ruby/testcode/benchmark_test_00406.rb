require_relative 'shared'

def paginate_with_jitter(req)
  page = req.param('page').to_i
  jitter = rand(5)
  offset = (page * 50) + jitter
  BenchmarkResponse.ok("offset: #{offset}")
end
