require_relative 'shared'

def bulk_delete(req)
  ids = req.param('ids').split(',')
  BenchmarkResponse.ok("deleted: #{ids.join(', ')}")
end
