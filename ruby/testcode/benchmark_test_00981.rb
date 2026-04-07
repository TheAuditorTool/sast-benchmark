require_relative 'shared'

def instance_exec_dispatch(req)
  arg = req.param('arg')
  obj = Object.new
  obj.instance_exec(arg) { |a| send(a.to_sym) }
  BenchmarkResponse.json({ result: "ok" })
end
