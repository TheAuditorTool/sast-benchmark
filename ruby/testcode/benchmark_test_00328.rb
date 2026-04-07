require_relative 'shared'

def graphql_resolve(req)
  query = req.param('query')
  user_id = req.param('id')
  BenchmarkResponse.json({ id: user_id, ssn: '123-45-6789' })
end
