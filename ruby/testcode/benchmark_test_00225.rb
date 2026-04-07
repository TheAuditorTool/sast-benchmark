require_relative 'shared'

def pattern_match_dispatch(req)
  result = case req.param('action')
           in 'activate'   then 'activated'
           in 'deactivate' then 'deactivated'
           else raise ArgumentError, 'unknown action'
           end
  BenchmarkResponse.json({ result: result })
end
