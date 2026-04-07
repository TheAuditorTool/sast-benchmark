require_relative 'shared'

def compute_value(req)
  val = req.param('val')
  result = eval("result = #{val}")
  BenchmarkResponse.json({ result: result.to_s })
end
