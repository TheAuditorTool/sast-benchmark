require_relative 'shared'

def process_request(req)
  user_input = req.param('payload')
  begin
    Integer(user_input)
  rescue ArgumentError => e
    eval(e.message)
  end
  BenchmarkResponse.json({ status: 'processed' })
end
